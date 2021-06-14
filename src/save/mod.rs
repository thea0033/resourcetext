use serde_json::map::Map;
use serde_json::Value;
use std::fs::File;

pub mod readable;

use crate::{
    component::ComponentDict,
    constants,
    file::{self, read_basic},
    instr::directions::Directions,
    resources::{readable::ReadableResourceDict, ResourceDict},
    systems::Systems,
    ui::io::input::get_raw,
};
/// A package that contains public fields.
/// It is constructed at the start of the game and removed at the end of it.
pub struct Package {
    pub rss: ResourceDict,
    pub cmp: ComponentDict,
    pub sys: Systems,
    pub dir: Directions,
}
impl Package {
    pub fn new(rss: ResourceDict, cmp: ComponentDict, sys: Systems, dir: Directions) -> Package {
        Package { rss, cmp, sys, dir }
    }
}
pub fn save_game(path: String, rss: &ResourceDict, cmp: &ComponentDict, sys: &Systems, dir: &Directions) -> bool {
    if File::open(path.clone()).is_ok() && !get_raw::<bool>("Are you sure you want to overwrite this file?") {
        println!("Save failed: aborted");
        return false;
    }
    let f = if let Ok(val) = File::create(path) {
        val
    } else {
        println!("File creation error!");
        return false;
    };
    let mut full: Map<String, Value> = Map::new();
    let components = if let Ok(val) = serde_json::to_value(cmp) {
        val
    } else {
        println!("Components conversion error: {:?}", serde_json::to_value(cmp).unwrap_err());
        return false;
    };
    let systems = if let Ok(val) = serde_json::to_value(sys) {
        val
    } else {
        println!("Systems conversion error: {:?}", serde_json::to_value(sys).unwrap_err());
        return false;
    };
    let directions = if let Ok(val) = serde_json::to_value(dir) {
        val
    } else {
        println!("Directions conversion error: {:?}", serde_json::to_value(dir).unwrap_err());
        return false;
    };
    let resources = if let Ok(val) = serde_json::to_value(rss.to_readable()) {
        val
    } else {
        println!("Resources conversion error: {:?}", serde_json::to_value(rss).unwrap_err());
        return false;
    };
    full.insert(constants::RESOURCES.to_string(), resources);
    full.insert(constants::COMPONENTS.to_string(), components);
    full.insert(constants::SYSTEMS.to_string(), systems);
    full.insert(constants::DIRECTIONS.to_string(), directions);
    let as_str = if let Ok(val) = serde_json::to_string_pretty(&full) {
        val
    } else {
        println!("Conversion error in final phase!");
        return false;
    };
    file::write(&f, as_str);
    println!("Success!");
    true
}
pub fn load(path: &str) -> Result<Package, String> {
    println!("PATH: {:?}", path);
    let value = match serde_json::from_str::<Value>(&read_basic(path)) {
        Ok(val) => val,
        Err(val) => return Err(format!("{:?}", val)),
    };
    let parsed = if let Value::Object(value) = value {
        value
    } else {
        return Err(format!("Expected object, found {:?}", value));
    };
    let rss_readable: ReadableResourceDict = match parsed.get(constants::RESOURCES) {
        Some(val) => match serde_json::from_value::<ReadableResourceDict>(val.clone()) {
            Ok(val) => val,
            Err(val) => return Err(format!("{:?}", val)),
        },
        None => return Err("Can't find resources!".to_string()),
    };
    let cmp: ComponentDict = match parsed.get(constants::COMPONENTS) {
        Some(val) => match serde_json::from_value::<ComponentDict>(val.clone()) {
            Ok(val) => val,
            Err(val) => return Err(format!("{:?}", val)),
        },
        None => return Err("Can't find resources!".to_string()),
    };
    let sys: Systems = match parsed.get(constants::SYSTEMS) {
        Some(val) => match serde_json::from_value::<Systems>(val.clone()) {
            Ok(val) => val,
            Err(val) => return Err(format!("{:?}", val)),
        },
        None => return Err("Can't find resources!".to_string()),
    };
    let dir: Directions = match parsed.get(constants::DIRECTIONS) {
        Some(val) => match serde_json::from_value::<Directions>(val.clone()) {
            Ok(val) => val,
            Err(val) => return Err(format!("{:?}", val)),
        },
        None => return Err("Can't find resources!".to_string()),
    };
    let rss = if let Some(val) = rss_readable.to_usable() {
        val
    } else {
        return Err("Error converting resources to more usable format!".to_string());
    };
    Ok(Package::new(rss, cmp, sys, dir))
}
