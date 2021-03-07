pub mod constants;
pub mod context;
pub mod docs;
pub mod keys;
pub mod options;
pub mod readable;

use self::options::OptionTable;
use self::{context::Context, docs::InfoDocs, keys::Keys};

use super::io::input::Buffer;

#[derive(Debug)]
pub enum MenuResult {
    Continue,
    Exit,
    Copy,
    Paste,
    Enter(usize),
}
#[derive(Clone, Copy, Debug)]
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
    New,
    Remove,
}
impl InputResult {
    pub fn from_int(a: i32) -> InputResult {
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
            19 => New,
            20 => Remove,
            _ => panic!("This doesn't represent a valid pattern!"),
        }
    }
}
pub fn grab(options: &OptionTable, page: usize, keys: &Keys, b: &mut Buffer) -> InputResult {
    options.print(page, keys);
    InputResult::from_int(keys.find(b.read()))
}
pub fn grab_menu_res(options: &OptionTable, config: &mut Config) -> MenuResult {
    let mut page: usize = 0;
    loop {
        let result = grab(options, page, &config.keys, &mut config.buffer);
        let id = result as usize;
        match result {
            InputResult::Invalid => {
                println!("You entered something invalid! ");
                config.buffer.flush();
            }
            _ if id < 10 => return MenuResult::Enter(id + page * 10),
            InputResult::Exit => return MenuResult::Exit,
            InputResult::Tick => {
                //do stuff (not implemented yet) TODO: Implement
                return MenuResult::Continue;
            }
            InputResult::Info => {
                docs::doc_menu(&InfoDocs::new("assets\\config\\docs.json").doc(), config, "Docs master".to_string());
            }
            InputResult::Configure => {
                //do stuff (not implemented yet) TODO: Implement
            }
            InputResult::Copy => return MenuResult::Copy,
            InputResult::Paste => return MenuResult::Paste,
            InputResult::Up => {
                if page < options.pages() - 1 {
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
#[derive(Clone)]
pub struct Config {
    pub buffer: Buffer,
    pub keys: Keys,
    pub context: Context,
}
pub fn sample_menu(config: &mut Config) {
    let mut n_list: Vec<String> = Vec::new();
    for i in 0..1000 {
        n_list.push(format!("{:?}", i));
    }
    let options = OptionTable::new(String::new(), n_list, config.context.grab(0));
    let res: MenuResult = grab_menu_res(&options, config);
    println!("{:?}", res);
}
