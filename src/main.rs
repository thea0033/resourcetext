#![allow(clippy::ptr_arg)]
#![allow(dead_code)]

use ui::menu::{config::Config};

mod component;
mod constants;
mod extra_bits;
mod file;
mod init;
mod instr;
mod location;
mod merge;
mod object;
mod resources;
mod save;
#[allow(dead_code)]
mod shape;
mod system;
mod systems;
mod ui;
pub fn main() {
    let mut pkg = init::generate_package(vec!["assets\\base\\"]).unwrap();
    let mut config: Config = Config::new("assets\\config\\".to_string()).unwrap();
    pkg.systems_menu(&mut config) 
}
