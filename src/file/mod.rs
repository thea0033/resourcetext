use crate::ui::io::ansi;
use crate::ui::io::input::get_str_raw;
use io::BufReader;
use std::{fs, io::Read};
use std::{
    fs::File,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
};
#[derive(Debug, Clone)]
pub struct FilePresets {
    asset_path: String,
}
impl FilePresets {
    pub fn new(asset_path: String) -> FilePresets {
        FilePresets { asset_path }
    }
    pub fn path(&self) -> String {
        self.asset_path.clone()
    }
}
pub fn read_file(path: &str, presets: &FilePresets) -> Vec<String> {
    let mut real_path = presets.asset_path.to_string();
    real_path.push_str(path);
    let mut result = read_lines(real_path);
    remove_extras(&mut result);
    result
}
fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut result: Vec<String> = Vec::new();
    loop {
        let mut s: String = String::new();
        let line = reader.read_line(&mut s);
        result.push(s.clone());
        if line.is_err() {
            return result;
        }
        if s.is_empty() {
            return result;
        }
    }
}
pub fn read_basic(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut result: String = String::new();
    file.read_to_string(&mut result).expect("Error reading!");
    result
}
pub fn ensure_file_exists(path: &str, presets: &FilePresets) {
    let path = presets.asset_path.clone() + path;
    if File::open(&path).is_err() {
        File::create(path).unwrap();
    }
}
pub fn write<T>(file: &File, contents: T)
where
    T: ToString,
{
    let mut f = BufWriter::new(file);
    f.write_all(contents.to_string().as_bytes()).expect("Unable to write data");
    f.flush().unwrap();
}
pub fn read_folder(name: &str) -> Vec<Vec<String>> {
    let val = fs::read_dir(name).expect("Couldn't do this for some reason");
    let mut result = val.map(|x| read_lines(x.unwrap().path())).collect();
    for line in &mut result {
        remove_extras(line);
    }
    result
}
pub fn remove_extras(v: &mut Vec<String>) {
    for s in v {
        if let Some('\n') = s.chars().next_back() {
            s.pop(); //Removes newline character
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop(); //Removes carriage return character
        }
    }
}
pub fn get_file(current: &str) -> io::Result<String> {
    let mut path: String = current.to_string();
    loop {
        print!("{}", ansi::RESET);
        println!("Your current path: {}", path);
        println!("Select a file from the current directory by typing it. ");
        println!("Typing a directory (marked in blue) will enter it. ");
        println!("Typing \".\" will exit the current directory if possible. ");
        println!("Typing a file that doesn't exist may prompt a new file to be created.");
        println!("Files: ");
        let mut folders: Vec<String> = Vec::new();
        let temp = fs::read_dir(&path)?;
        for line in temp {
            let line = line?;
            let typ = line.file_type()?;
            if typ.is_dir() {
                print!("{}", ansi::BLUE);
                folders.push(line.file_name().to_str().unwrap().to_string());
            } else {
                print!("{}", ansi::GREEN);
            }
            println!("{}", line.file_name().to_str().unwrap());
        } // prints out the current directory
        let input = get_str_raw();
        if input == "." {
            let mut temp = path.split('\\').collect::<Vec<&str>>();
            println!("{:?}", temp);
            temp.pop();
            temp.pop();
            path = temp.join("\\");
            path.push('\\');
        } else if folders.contains(&input) {
            path.push_str(&input);
            path.push('\\');
        } else {
            return Ok(path + &input);
        }
    }
}
