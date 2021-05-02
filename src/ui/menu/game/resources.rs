use crate::{resources::ResourceID, save::Package, ui::menu::config::Config};

impl Package {
    pub fn select_resources(&mut self, config: &mut Config) -> Vec<u64> {
        let res: Vec<u64> = vec![0; self.rss.len()];
        while let Some(val) = self.sel
        res
    }
    pub fn select_resource(&mut self, config: &mut Config) -> ResourceID {
        
    }
}