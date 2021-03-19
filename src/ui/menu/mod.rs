pub mod config;
pub mod constants;
pub mod context;
pub mod docs;
pub mod escape;
pub mod game;
pub mod keys;
pub mod options;
pub mod readable;

use ui::io::input::{get_raw, get_str_raw};

use crate::save::Package;
use crate::*;

use self::{config::Config, options::OptionTable};
use self::{docs::InfoDocs, keys::Keys};

use super::io::input::Buffer;

#[derive(Debug)]
pub enum MenuResult {
    Continue,
    Exit,
    Copy,
    Paste,
    Enter(usize),
    New,
    Remove,
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
            18 => New,
            19 => Remove,
            _ => panic!("This doesn't represent a valid pattern!"),
        }
    }
}
pub fn grab(options: &OptionTable, page: usize, keys: &Keys, b: &mut Buffer) -> InputResult {
    options.print(page, keys);
    InputResult::from_int(keys.find(b.read()))
}
pub fn grab_menu_res(options: &OptionTable, config: &mut Config, pkg: &mut Package) -> MenuResult {
    let mut page: usize = 0;
    loop {
        let result = grab(options, page, &config.keys, &mut config.buffer);
        let id = result as usize;
        match result {
            InputResult::Invalid => {
                wait_for_user(config, "You entered a too high value! Press enter to continue: ");
            }
            _ if id < 10 => {
                if id + page * 10 < options.len() {
                    return MenuResult::Enter(id + page * 10);
                } else {
                    wait_for_user(config, "You entered something invalid! Press enter to continue: ");
                }
            }
            InputResult::Exit => return MenuResult::Exit,
            InputResult::Tick => {
                pkg.tick();
                return MenuResult::Continue;
            }
            InputResult::Info => {
                docs::doc_menu(&InfoDocs::new("assets\\config\\docs.json").doc(), config, "Docs master".to_string());
            }
            InputResult::Configure => {
                config.configure_keys();
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
            InputResult::New => return MenuResult::New,
            InputResult::Remove => return MenuResult::Remove,
            _ => {
                panic!("Something went horribly wrong!")
            }
        }
    }
}
pub fn grab_menu_res_restricted(options: &OptionTable, config: &mut Config) -> MenuResult {
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
                println!("You entered something invalid! ");
                config.buffer.flush();
            }
            InputResult::Info => {
                docs::doc_menu(&InfoDocs::new("assets\\config\\docs.json").doc(), config, "Docs master".to_string());
            }
            InputResult::Configure => config.configure_keys(),
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
            InputResult::New => return MenuResult::New,
            InputResult::Remove => return MenuResult::Remove,
            _ => {
                panic!("Something went horribly wrong!")
            }
        }
    }
}

pub fn sample_menu(config: &mut Config) {
    let mut n_list: Vec<String> = Vec::new();
    for i in 0..1000 {
        n_list.push(format!("{:?}", i));
    }
    let options = OptionTable::new(String::new(), n_list, config.context.grab(0));
    let res: MenuResult = grab_menu_res_restricted(&options, config);
    println!("{:?}", res);
}
pub fn wait_for_user(config: &mut Config, message: &str) {
    config.buffer.flush();
    println!("{}", message);
    get_str_raw();
}
