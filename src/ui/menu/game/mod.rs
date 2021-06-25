use std::{io::{self, Error, ErrorKind}, path::PathBuf, sync::mpsc::channel};

use crate::{init::{self, generate_package}, save::{self, load, Package}, ui::{io::{
        ansi,
        input::{get_raw, get_str_raw, refresh},
    }, menu::graphics::loading_screen}};

use super::{config::Config, constants, grab_menu_res, options::OptionTable, MenuResult};

pub mod component;
pub mod instr;
pub mod instrs;
pub mod misc;
pub mod object;
pub mod queue;
pub mod recipe;
pub mod resources;
pub mod select;
pub mod system;
pub mod systems;
pub mod tick;
pub fn start_program(config: &mut Config) -> io::Result<()> {
    let (send, recv) = channel();
    let v = std::thread::spawn(|| loading_screen(recv, 11, 300, 20));
    for i in 0..=10 {
        let _ = send.send(Some(i.to_string() + "0%"));
    }
    v.join().expect("FAILED");
    println!("Welcome to resourcetext!");
    println!("This is more of an engine than a game. It's designed to be easily modified.");
    println!("For now, I'm assuming you want to play the simple version that comes pre-installed.");
    println!("Detailed help will be in the information section. ");
    println!("You will now be prompted to select multiple folders that store, well, game asset files. ");
    println!("The current one is 'base'. You'll want to use this clumsy user interface to navigate to base.");
    println!("Then, after that, type the name of any file inside base. This will select the directory.");
    let mut package = start_game(config)?;
    package.menu(config)
}
pub fn start_game(config: &mut Config) -> io::Result<Package> {
    let response = loop {
        println!("You can either load a game or start a new game. Enter 'l' to load a game, and enter 'n' to create a new game. ");
        match config.buffer.read() {
            'l' => break true,
            'n' => break false,
            _ => {}
        }
    };
    config.buffer.safety(); // prevents undefined behavior
    if response {
        // loading a game
        match select_file(current_dir()) {
            Ok(file) => load(&file).map_err(|x| Error::new(ErrorKind::InvalidInput, x)),
            Err(val) => Err(val),
        }
    } else {
        // new game
        match select_folders(current_dir()) {
            Ok(folders) => generate_package(folders).map_err(|x| Error::new(ErrorKind::InvalidInput, x)),
            Err(val) => Err(val),
        }
    }
}
pub fn current_dir() -> PathBuf {
    let val = std::env::current_dir();
    if let Ok(val) = val {
        val
    } else {
        PathBuf::from("\\")
    }
}
pub fn select_folders(mut orig: PathBuf) -> io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    loop {
        println!("COMMANDS: e - enter [folder], s - select [file or folder], x - exit current folder");
        println!("q - quit with current folders, d [number] - deselect the current folder");
        let val = std::fs::read_dir(orig.clone())?;
        println!("Current directory: {:?}", orig);
        println!("Files are in blue, and folders are in green.");
        for line in val {
            if let Ok(val) = line {
                if val.file_type()?.is_dir() {
                    print!("{}", ansi::GREEN);
                } else {
                    print!("{}", ansi::BLUE);
                }
                println!("{}{}", val.file_name().to_str().unwrap_or("UNVIEWABLE FILE"), ansi::RESET);
            }
        }
        println!("-----------------------------------------------------------------------------------");
        println!("Currently selected folders: ");
        for line in &result {
            println!("{}", line);
        }
        let x = get_str_raw();
        refresh();
        if x.starts_with("e ") {
            orig.push(x.split_at(2).1);
        } else if x.starts_with("s ") {
            result.push(orig.to_string_lossy().to_string() + "\\" + x.split_at(2).1 + "\\");
        } else if x.starts_with("x") {
            orig.pop();
        } else if x.starts_with("q") {
            return Ok(result);
        } else if x.starts_with("d ") {
            if let Ok(val) = x.split_at(2).1.parse::<usize>() {
                if val < result.len() {
                    result.remove(val);
                }
            }
        }
    }
}
pub fn select_file(mut orig: PathBuf) -> io::Result<String> {
    loop {
        println!("COMMANDS: e - enter [folder], s - select [file], x - exit current folder");
        let val = std::fs::read_dir(orig.clone())?;
        println!("Current directory: {:?}", orig);
        println!("Files are in blue, and folders are in green.");
        for line in val {
            if let Ok(val) = line {
                if val.file_type()?.is_dir() {
                    print!("{}", ansi::GREEN);
                } else {
                    print!("{}", ansi::BLUE);
                }
                println!("{}{}", val.file_name().to_str().unwrap_or("UNVIEWABLE FILE"), ansi::RESET);
            }
        }
        let x = get_str_raw();
        refresh();
        if x.starts_with("e ") {
            orig.push(x.split_at(2).1);
        } else if x.starts_with("s ") {
            return Ok(orig.to_string_lossy().to_string() + "\\" + x.split_at(2).1);
        } else if x.starts_with("x") {
            orig.pop();
        }
    }
}
impl Package {
    pub fn illegal_state(&self, cfg: &mut Config) {
        cfg.buffer.safety();
        println!("Illegal state reached!");
        cfg.buffer.safety();
    }
    pub fn menu(&mut self, config: &mut Config) -> io::Result<()> {
        loop {
            let table: OptionTable = OptionTable::new(
                String::new(),
                vec!["Continue/enter the game".to_string()],
                config.context.grab(constants::START),
            );
            match grab_menu_res(&table, config, self) {
                MenuResult::Exit => {
                    if config.buffer.get_safety(
                        "Are you sure you want to quit? This does NOT save your game.",
                        "Please enter true or false. ",
                    ) {
                        break;
                    }
                }
                MenuResult::New => {
                    save::save_game(select_file(current_dir())?, &self.rss, &self.cmp, &self.sys, &self.dir);
                }
                MenuResult::Remove => {
                    if config.buffer.get_safety(
                        "Are you sure you want to load/create a game? This does NOT save your game.",
                        "Please enter true or false. ",
                    ) {
                        *self = start_game(config)?;
                        self.systems_menu(config);
                    }
                }
                MenuResult::Enter(0) => {
                    self.systems_menu(config);
                }
                _ => {}
            }
        }
        Ok(())
    }
}
