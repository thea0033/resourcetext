#![allow(clippy::ptr_arg)]
#![allow(dead_code)]

use location::Location;
use systems::system_id::SystemID;
use ui::menu::config::Config;

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
    for i in 0..19 {
        pkg.sys
            .add_object(&pkg.rss, &pkg.cmp, &mut pkg.dir, i.to_string(), Location::new(0.0, 0.0), SystemID::new(0));
    }
    let mut config: Config = Config::new("assets\\config\\".to_string()).unwrap();
    pkg.systems_menu(&mut config)
}
