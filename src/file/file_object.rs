use std::{iter::Zip, slice::Iter};

const WHITESPACE: &str = "    ";
const WHITESPACE_LEN: usize = 4;
#[derive(Clone, Debug, PartialEq)]
pub struct FileObject {
    name: String,
    contents: Vec<FileObject>,
    names: Vec<String>,
}
impl FileObject {
    pub fn read_from(file: Vec<String>, name: String, tabs: usize) -> FileObject {
        let mut contents: Vec<FileObject> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        let mut buffer: Vec<String> = Vec::new();
        let mut buffer_name: String = "".to_string();
        for mut line in file {
            if line.len() >= WHITESPACE_LEN && &line[0..WHITESPACE_LEN] == WHITESPACE {
                //If an indent exists...
                line.replace_range(0..WHITESPACE_LEN, ""); //Removes the indent
                buffer.push(line); //Adds the line
            } else {
                //If there's no indent...
                let temp: Vec<&str> = line.split(':').collect();
                if temp.len() == 1 {
                    contents.push(FileObject::read_from(buffer, buffer_name, tabs + 1));
                    names.push(temp[0].to_string());
                    buffer_name = temp[0].to_string();
                } else {
                    contents.push(FileObject::read_from(buffer, buffer_name, tabs + 1));
                    buffer_name = temp[1].to_string();
                    names.push(temp[0].to_string());
                }
                buffer = Vec::new();
            }
        }
        if !contents.is_empty() {
            contents.push(FileObject::read_from(buffer, buffer_name, tabs + 1)); //Adds the last bit
            contents.remove(0); //Removes the first object (which doesn't contain anything; this is out of sync)
        }
        FileObject { name, contents, names }
    }
    pub fn blank(name: String) -> FileObject {
        FileObject {
            name,
            contents: Vec::new(),
            names: Vec::new(),
        }
    }
    pub fn merge(&mut self, other: FileObject) {
        for (name, object) in other.names.into_iter().zip(other.contents.into_iter()) {
            if let Some(val) = self.names.iter().position(|x| **x == name) {
                self.contents[val].merge(object);
            } else {
                self.names.push(name.clone());
                self.contents.push(object.clone());
            }
        }
    }
    pub fn get(&self, name: &str) -> Option<&FileObject> {
        for (i, line) in self.names.iter().enumerate() {
            if line == name {
                return Some(&self.contents[i]);
            }
        }
        None
    }
    pub fn grab_contents(&self) -> Zip<Iter<String>, Iter<FileObject>> {
        let v = self.names.iter().zip(self.contents.iter());
        v
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}
