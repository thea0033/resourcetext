use crate::file;

use super::readable::ReadableKeys;

#[derive(Clone)]
pub struct Keys {
    keys: Vec<char>,
    visible: Vec<bool>,
}
impl Keys {
    pub fn create(keys: Vec<char>, visible: Vec<bool>) -> Keys {
        Keys { keys, visible }
    }
    pub fn find(&self, input: char) -> i32 {
        if let Some(val) = self.keys.iter().position(|x| *x == input) {
            val as i32
        } else {
            -1
        }
    }
    pub fn new(p: &str) -> Result<Keys, serde_json::Error> {
        let s = file::read_basic(p);
        Ok(serde_json::from_str::<ReadableKeys>(&s)?.grab())
    }
    pub fn visible(&self, i: usize) -> bool {
        self.visible[i]
    }
    pub fn key(&self, i: usize) -> char {
        self.keys[i]
    }
    pub fn set(&mut self, i: usize, new: char) {
        self.keys[i] = new;
    }
    pub fn set_visible(&mut self, i: usize, new: bool) {
        self.visible[i] = new;
    }
    pub fn test(&mut self, pos: usize, new: char) -> bool {
        for (i, line) in self.keys.iter().enumerate() {
            if pos != i && new == *line {
                return false;
            }
        }
        true
    }
    pub fn find_duplicate(&mut self, excl: usize) -> Option<usize> {
        for (i, line) in self.keys.iter().enumerate() {
            if excl != i && self.keys[excl] == *line {
                return Some(i);
            }
        }
        None
    }
}
pub fn is_yes(v: char) -> bool {
    v == 'y' || v == 'Y'
}
pub fn is_no(v: char) -> bool {
    v == 'n' || v == 'N'
}
