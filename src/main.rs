#![allow(clippy::ptr_arg)]




use file::FilePresets;
use ui::{io::input::Buffer, menu::{Config, keys::Keys, context::Context}};

use crate::systems::readable::ReadableSystems;
mod merge;
mod component;
mod constants;
mod extra_bits;
mod file;
mod init;
mod instr;
mod location;
mod object;
mod resources;
mod save;
#[allow(dead_code)]
mod shape;
mod system;
mod systems;
mod ui;
pub fn main() {
    let buffer = Buffer::new();
    let keys:Keys = Keys::new("assets\\config\\keys.json").unwrap();
    let context = Context::new("assets\\config\\context.json").expect("Whatever");
    println!("{:?}", context);
    let mut config:Config = Config {
        buffer,
        keys,
        context 
    };
    ui::menu::sample_menu(&mut config);
}