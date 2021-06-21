#![allow(clippy::ptr_arg)]
#![allow(dead_code)]

use ui::menu::{config::Config, game::start_program};

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
    let mut config: Config = Config::new("assets\\config\\".to_string()).unwrap();
    start_program(&mut config).expect("An error occurred!");
}
