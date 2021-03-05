use std::cmp::min;

use super::io::input::Buffer;

pub enum MenuResult {
    Continue,
    Exit, 
    Tick, 
    Copy, 
    Paste,
    Enter(usize),
}
#[derive(Clone, Copy)]
pub enum InputResult {
    Invalid = -1,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Exit,
    Tick,
    Info,
    Configure,
    Copy,
    Paste,
    Up,
    Down,
    Save,
}
impl InputResult {
    pub fn from_int(a:i32) -> InputResult {
        use InputResult::*;
        match a {
            -1 => Invalid,
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            10 => Exit,
            11 => Tick,
            12 => Info,
            13 => Configure,
            14 => Copy,
            15 => Paste,
            16 => Up,
            17 => Down,
            18 => Save,
            _ => panic!("This doesn't represent a valid pattern!")
        }
    }
}
pub struct Keys {
    keys:Vec<char>
}
impl Keys {
    pub fn create(defaults: Vec<char>) -> Keys {
        Keys {keys:defaults}
    }
    pub fn find(&self, input:char) -> i32 {
        if let Some(val) = self.keys.iter().position(|x| *x == input) {
            val as i32
        } else {
            -1
        }
    }
}
pub struct OptionTable {
    others: String,
    numbered: Vec<String>,
    pages: usize
}
impl OptionTable {
    pub fn print(&self, page:usize) {
        println!("{}", self.others);
        println!();
        for i in (self.pages * 10)..min((self.pages + 1) * 10, self.numbered.len()) {
            println!("{}. {}", i % 10, self.numbered[i]);
        }
    }
    pub fn new(others: String, numbered: Vec<String>) -> OptionTable {
        OptionTable {
            others, 
            pages: (numbered.len() + 9 ) / 10,
            numbered,
        }
    }
}
pub fn grab(options: &OptionTable, page: usize, keys: &Keys, a:char) -> InputResult {
    options.print(page);
    InputResult::from_int(keys.find(a))
}
pub fn grab_menu_res(options: &OptionTable, keys: &mut Keys, b: &mut Buffer) -> MenuResult {
    let mut page:usize = 0;
    loop {
        let result = grab(options, page, keys, b.read());
        let id = result as usize;
        match result {
            InputResult::Invalid => {
                println!("You entered something invalid! ");
                b.flush();
            }
            _ if id < 10 => {
                return MenuResult::Enter(id)
            }
            InputResult::Exit => {return MenuResult::Exit}
            InputResult::Tick => {
                //do stuff (not implemented yet) TODO: Implement 
                return MenuResult::Continue
            }
            InputResult::Info => {
                //do stuff (not implemented yet) TODO: Implement 
            }
            InputResult::Configure => {
                //do stuff (not implemented yet) TODO: Implement 
            }
            InputResult::Copy => {
                return MenuResult::Copy
            }
            InputResult::Paste => {
                return MenuResult::Paste
            }
            InputResult::Up => {
                if page < options.pages - 1{
                    page += 1;
                }
            }
            InputResult::Down => {
                if page > 0 {
                    page -= 1;
                }
            }
            _ => {}
        }
    }
}

pub fn sample_menu(keys: &mut Keys, b: &mut Buffer) {
    let options = OptionTable::new("q. Quit".to_string(), vec!["1. die".to_string(), "2. live".to_string()]);
    let res:MenuResult = grab_menu_res(&options, keys, b);


}