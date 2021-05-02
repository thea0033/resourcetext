use crate::{init::generate_package, save::Package};

use super::config::Config;

pub mod component;
pub mod instr;
pub mod instrs;
pub mod object;
pub mod queue;
pub mod recipe;
pub mod resources;
pub mod select;
pub mod system;
pub mod systems;
pub mod tick;
pub mod misc;
pub fn start_game(config: &mut Config) {
    println!("Welcome to resourcetext!");
    println!("You are now breathing manually. ");
    println!("This is more of an engine than a game. It's designed to be easily modified.");
    println!("For now, I'm assuming you want to play the simple version that comes pre-installed.");
    println!("Detailed help will be in the information section. ");
    println!("You will now be prompted to select multiple folders that store, well, game asset files. ");
    println!("The current one is 'base'. You'll want to use this clumsy user interface to navigate to base.");
    println!("Then, after that, type the name of any file inside base. This will select the directory.");
}