use crate::file;

use super::{constants, Config, MenuResult, OptionTable};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InfoDocs {
    contents: InfoDoc,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub enum InfoDoc {
    Menu(Vec<String>, Vec<InfoDoc>),
    Endpoint(Vec<String>),
}
impl InfoDocs {
    pub fn new(path: &str) -> InfoDocs {
        let file = file::read_basic(path);
        serde_json::from_str(&file).unwrap()
    }
    pub fn doc(&self) -> &InfoDoc {
        &self.contents
    }
}
pub fn doc_menu(doc: &InfoDoc, config: &mut Config, name: String) -> MenuResult {
    loop {
        match doc {
            InfoDoc::Menu(val1, val2) => {
                let list = val1.clone();
                let options = OptionTable::new(name.clone(), list, config.context.grab(constants::INFO_CONTEXT));
                let res: super::MenuResult = super::grab_menu_res(&options, config);
                match res {
                    super::MenuResult::Continue => continue,
                    super::MenuResult::Exit => break MenuResult::Exit,
                    super::MenuResult::Enter(v) => break doc_menu(&val2[v], config, val1[v].clone()),
                    _ => (),
                }
            }
            InfoDoc::Endpoint(val) => {
                let mut newname = name.clone();
                let list = Vec::new();
                for line in val {
                    newname.push('\n');
                    newname.push_str(line);
                }
                let options = OptionTable::new(newname, list, config.context.grab(constants::INFO_CONTEXT));
                let res: super::MenuResult = super::grab_menu_res(&options, config);
                match res {
                    MenuResult::Exit => break MenuResult::Exit,
                    _ => {}
                }
            }
        }
    }
}
