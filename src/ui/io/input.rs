use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
    str::FromStr,
};
///Gets a raw string from stdin.
pub fn get_str_raw() -> String {
    let mut s: String = String::new();
    print!("-->");
    stdout().flush().expect("Some big problem!");
    stdin().read_line(&mut s).expect("Something went horribly wrong!");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
pub fn refresh() {
    for _ in 0..100 {
        println!();
    }
}
/// Gets a raw string from stdin and parses it into type T.
pub fn get_raw<T>(err: &str) -> T
where
    T: FromStr,
{
    loop {
        if let Ok(val) = get_str_raw().parse::<T>() {
            break val;
        }
        println!("{}", err);
    }
}
pub fn record<P>(to_record: String, mut w: P) -> String
where
    P: FnMut(String) -> String,
{
    w(to_record)
}
#[derive(Clone)]
pub struct Buffer {
    b: VecDeque<char>,
    sep: char,
}
impl Buffer {
    pub fn input(&mut self) {
        let mut f: VecDeque<char> = get_str_raw().chars().collect();
        if f.is_empty() {
            f.push_back('/');
        }
        self.b.append(&mut f);
        refresh();
    }
    /// reads from input - adds input if necessary. Otherwise, takes the first character in the buffer.
    pub fn read(&mut self) -> char {
        while self.b.is_empty() {
            self.input();
        }
        self.b.pop_front().expect("Safe unwrap")
    }
    /// removes all characters up to and including a / or a break in input and returns them.
    pub fn flush(&mut self) -> String {
        let mut result = String::new();
        if self.b.is_empty() {
            self.input();
        }
        while let Some(val) = self.b.pop_front() {
            if val == self.sep {
                break;
            }
            result.push(val);
        }
        return result;
    }
    /// Gets rid of the entire buffer. Used when the user does something wrong.
    pub fn safety(&mut self) -> String {
        let mut temp: VecDeque<char> = VecDeque::new();
        temp.append(&mut self.b);
        temp.into_iter().collect()
    }
    pub fn new(sep: char) -> Buffer {
        Buffer { b: VecDeque::new(), sep }
    }

    pub fn get_flush<T>(&mut self, msg: &str, err: &str) -> T
    where
        T: FromStr,
    {
        println!("{}", msg);
        loop {
            if let Ok(val) = self.flush().parse::<T>() {
                break val;
            }
            println!("{}", err);
        }
    }
    pub fn get_safety<T>(&mut self, msg: &str, err: &str) -> T
    where
        T: FromStr,
    {
        println!("{}", msg);
        loop {
            if self.b.is_empty() {
                self.input();
            }
            if let Ok(val) = self.safety().parse::<T>() {
                break val;
            }
            println!("{}", err);
        }
    }
    pub fn get_valid_flush<T, V>(&mut self, msg: &str, err: &str, valid: V) -> T
    where
        T: FromStr,
        V: Fn(&T) -> bool,
    {
        loop {
            let res = self.get_flush(msg, err);
            if valid(&res) {
                break res;
            }
            println!("{}", err);
        }
    }
}
