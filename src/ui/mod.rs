use crate::{component::Components, resources::ResourceDict, save::Package};

pub mod io;
pub mod menu;
impl Package{
    pub fn simple(&mut self) {
        req(&mut self.rss, &mut self.cmp)
    }

}
pub fn req(dict: &mut ResourceDict, cmp : &mut Components) {

}