use crate::{location::Location, save::Package, ui::menu::config::Config};

impl Package {
    pub fn select_location(&self, config: &mut Config) -> Location {
        println!("Enter the x coordinate of the location.");
        let x = config.buffer.get_flush("Please enter a valid number (decimals ok)");
        println!("Enter the y coordinate of the location.");
        let y = config.buffer.get_flush("Please enter a valid number (decimals ok)");
        Location::new(x, y)
    }
}