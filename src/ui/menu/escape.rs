use std::fs;

use crate::{
    file::get_file,
    save::{readable::SemiReadablePackage, Package},
    ui::io::input::get_raw,
};

use super::{config::Config, constants::ONLY_QUIT, grab_menu_res, options::OptionTable, wait_for_user};

impl Package {
    pub fn save(&self, config: &mut Config) -> Result<(), String> {
        loop {
            config.buffer.flush();
            let path = get_file("~");
            if let Ok(val) = path {
                let pkg: SemiReadablePackage = SemiReadablePackage::from(self);
                let v = serde_json::to_string(&pkg).map_err(|x| x.to_string())?;
                fs::write(val, v).map_err(|x| x.to_string())?;
            }
        }
    }
    pub const ESC_OPTIONS: &'static [&'static str] = &["save", "quit the game"];
    pub fn get_esc_options() -> Vec<String> {
        Package::ESC_OPTIONS.iter().map(|x| x.to_string()).collect()
    }
    pub fn esc_menu(&mut self, config: &mut Config) -> bool {
        loop {
            config.buffer.flush();
            let table: OptionTable = OptionTable::new(String::new(), Package::get_esc_options(), config.context.grab(ONLY_QUIT));
            match grab_menu_res(&table, config, self) {
                super::MenuResult::Continue => continue,
                super::MenuResult::Exit => break false,
                super::MenuResult::Enter(0) => {
                    if let Ok(()) = self.save(config) {
                        wait_for_user(config, "Your game was successfully saved!");
                    } else if let Err(val) = self.save(config) {
                        wait_for_user(config, &format!("An error occurred: {}\nYour game was not saved.", val));
                    }
                }
                super::MenuResult::Enter(1) => {
                    println!("Are you sure you want to quit? Note that this does not save your game. ");
                    if get_raw("Please enter a boolean (true or false). ") {
                        return true;
                    }
                }
                _ => {
                    println!("Invalid input!");
                }
            }
        }
    }
}
