use crate::{instr::condition::Condition, location::Location, save::Package, ui::menu::config::Config};

impl Package {
    pub fn select_location(&self, config: &mut Config) -> Location {
        let x = config
            .buffer
            .get_flush("Enter the x coordinate of the location.", "Please enter a valid number (decimals ok)");
        let y = config
            .buffer
            .get_flush("Enter the y coordinate of the location.", "Please enter a valid number (decimals ok)");
        Location::new(x, y)
    }
    pub fn select_condition(&self, config: &mut Config) -> Condition {
        todo!()
    }
}
