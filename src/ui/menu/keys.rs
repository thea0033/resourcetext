use crate::file;

use super::readable::ReadableKeys;

#[derive(Clone)]
pub struct Keys {
    keys:Vec<char>,
    visible:Vec<bool>,
}
impl Keys {
    pub fn create(keys: Vec<char>, visible:Vec<bool>) -> Keys {
        Keys {keys, visible}
    }
    pub fn find(&self, input:char) -> i32 {
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
    pub fn visible(&self, i:usize) -> bool {
        self.visible[i]
    }
    pub fn key(&self, i:usize) -> char {
        self.keys[i]
    }
}