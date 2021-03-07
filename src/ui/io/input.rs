use std::{collections::VecDeque, io::stdin, str::FromStr};
///Gets a raw string from stdin. 
pub fn get_str_raw() -> String {
    let mut s:String = String::new();
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
pub fn get_raw<T>(err: &str) -> T where T: FromStr{
    loop {
        if let Ok(val) = get_str_raw().parse::<T>() {
            break val;
        }
        println!("{}", err);
    }
}
pub fn record<P>(to_record:String, mut w: P) -> String where P:FnMut(String) -> String{
    w(to_record)
}
#[derive(Clone)]
pub struct Buffer {
    b:VecDeque<char>
}
impl Buffer {
    pub fn read(&mut self) -> char{
        while self.b.is_empty() {
            let mut f:VecDeque<char> = get_str_raw().chars().collect();
            if f.is_empty() {
                f.push_back('\n');
            }
            self.b.append(&mut f);
            refresh();
        }
        self.b.pop_front().expect("Safe unwrap")
    }
    pub fn flush(&mut self) {
        self.b.clear();
    }
    pub fn new() -> Buffer {
        Buffer {
            b:VecDeque::new()
        }
    }
}