use ui::menu::keys::{is_no, is_yes};

use crate::ui::io::input::Buffer;

use super::{MenuResult, context::Context, keys::Keys, options::OptionTable};
use crate::*;
use super::grab_menu_res_restricted;
#[derive(Clone)]
pub struct Config {
    pub buffer: Buffer,
    pub keys: Keys,
    pub context: Context,
}
impl Config {
pub fn new(p: String) -> Result<Self, serde_json::Error> {
    Ok(Self {
        buffer: Buffer::new(),
        keys: Keys::new(&(p.clone() + "keys.json"))?,
        context: Context::new(&(p + "context.json"))?,
    })
}
pub const OTHERS: &'static str = "Select a key to modify it or press q to quit.";
pub fn configure_keys(&mut self) {
    loop {
        self.buffer.flush();
        let others = Config::OTHERS.to_string();
        let numbered: Vec<String> = self.grab_key_list();
        let table: OptionTable = OptionTable::new(others, numbered, self.context.grab(crate::ui::menu::constants::ONLY_QUIT));
        match grab_menu_res_restricted(&table, self) {
            MenuResult::Exit => break,
            MenuResult::Enter(val) => self.configure_key_checked(val),
            _ => {
                println!("Invalid input!");
                self.buffer.flush();
            }
        }
    }
}
pub fn grab_key_list(&mut self) -> Vec<String> {
    self.context
        .grab(crate::ui::menu::constants::DISPLAY_KEYS)
        .into_iter() // forms an iterator
        .map(|x| x.unwrap())
        .collect()
}
pub fn display_key_list(&mut self) -> String {
    let mut res: String = String::new();
    for (i, line) in self.grab_key_list().iter().enumerate() {
        res.push_str(&format!("{}. {}\n", self.keys.key(i), line));
    }
    res
}
pub fn configure_key_checked(&mut self, id: usize) {
    self.configure_key(id);
    while let Some(val) = self.keys.find_duplicate(id) {
        self.configure_key_checked(val);
    }
}
pub fn configure_key(&mut self, id: usize) {
    println!("current key layout: \n{}", self.display_key_list());
    println!(
        "enter the new hotkey for {} (currently {}): ",
        self.grab_key_list()[id],
        self.keys.key(id)
    );
    let nc = self.buffer.read();
    if !self.keys.test(id, nc) {
        println!("changing this hotkey will result in conflicts that must be resolved. are you sure you want to do this? y/n");
        self.buffer.flush();
        if !is_yes(self.buffer.read()) {
            return;
        }
    }
    println!("do you want this key to show up? y/n");
    self.keys.set_visible(id, !is_no(self.buffer.read()));
    self.keys.set(id, nc);
    self.buffer.flush();
}
}