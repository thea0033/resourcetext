use crate::file;

use super::readable::ReadableContext;
#[derive(Debug, Clone)]
pub struct Context {
    context: Vec<Vec<Option<String>>>,
}

impl Context {
    pub fn grab(&self, id: usize) -> Vec<Option<String>> {
        self.context[id].clone()
    }
    pub fn default(&self) -> Vec<Option<String>> {
        self.context[0].clone()
    }
    pub fn new(p: &str) -> Result<Context, serde_json::Error> {
        let s = file::read_basic(p);
        let val = serde_json::from_str::<ReadableContext>(&s)?.get();
        let refined:Vec<Vec<Option<String>>> = val.into_iter().map(|x| x.1.into_iter().map(|x| x.1).collect()).collect();

        Ok(Context {context:refined})
    }
}