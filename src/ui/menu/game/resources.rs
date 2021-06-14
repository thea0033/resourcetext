use crate::{
    resources::ResourceID,
    save::Package,
    ui::menu::{config::Config, constants, options::OptionTable},
};

impl Package {
    pub fn select_resources(&mut self, config: &mut Config, res: Option<Vec<u64>>) -> Vec<u64> {
        let mut res: Vec<u64> = res.unwrap_or(vec![0; self.rss.len()]);
        while let Some(val) = self.select_resource(config) {
            res[val.get()] = config.buffer.get_flush("Please enter the amount of resources.", "Please enter a number!");
        }
        res
    }
    pub fn select_resource(&mut self, config: &mut Config) -> Option<ResourceID> {
        let table = OptionTable::new(
            "Select a resource:".to_string(),
            self.rss.display(),
            config.context.grab(constants::SELECT),
        );
        self.generic_select(config, &table, None, |x| ResourceID::new(x))
    }
}
