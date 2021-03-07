use crate::{
    component::readable::ReadableComponents, constants::*, file::read_basic, resources::readable::ReadableResourceDict, save::Package,
    systems::readable::ReadableSystems,
};
use std::{collections::HashMap, str::FromStr};

use crate::component::recipe::Recipe;
use crate::{
    component::{Component, Components},
    file::{file_object::FileObject, read_folder},
    instr::Directions,
    location::Location,
    object::Object,
    resources::{ResourceDict, ResourceID},
    systems::{system_id::SystemID, Systems},
};
pub fn generate_package(path: &str) -> Result<Package, String> {
    let rss = rss(path)?;
    let cmp = cmp(&rss, path)?;
    let mut dir = Directions::new();
    let sys = sys(&rss, &cmp, &mut dir, path)?;
    Ok(Package::new(rss, cmp, sys, dir))
}

pub fn sys(rss: &ResourceDict, cmp: &Components, dir: &mut Directions, path: &str) -> Result<Systems, String> {
    let file = format!("{}\\{}", path, SYSTEMS);
    let readable: ReadableSystems = crate::extra_bits::result_compat(serde_json::from_str(&read_basic(&file)), |x| format!("{:?}", x))?;
    readable.convert(rss, cmp, dir)
}
pub fn sys_old(rss: &ResourceDict, cmp: &Components, dir: &mut Directions, file: &FileObject) -> Systems {
    let mut s = Systems::new();
    if let Some(val) = file.get(SYSTEMS) {
        //If there's a systems object in the file object...
        for (name, line) in val.grab_contents() {
            //For every system in the systems object...
            s.add_system(name.to_string(), grab_location(line)); //Adds the system based on the location grabbed
            if let Some(val) = line.get(OBJECTS) {
                //If there's an objects object in the systems object...
                for (name, line) in val.grab_contents() {
                    //For every object in the objects object...
                    let temp = s.add_object(rss, cmp, dir, name.to_string(), grab_location(line), SystemID::new(s.len() - 1));
                    init_object(line, s.get_object_mut(temp), cmp, rss);
                }
            } else {
                panic!("Can't find objects!");
            }
        }
    } else {
        println!("Couldn't find systems object!");
    }
    s
}
pub fn init_object(file: &FileObject, obj: &mut Object, cmp: &Components, rss: &ResourceDict) {
    if let Some(val) = file.get(COMPONENTS) {
        if let Some(val) = val.get(ACCESSIBLE) {
            for (name, line) in val.grab_contents() {
                let component = cmp.get_from_name(name);
                let amt = parse(line, usize::MAX);
                obj.force_install_components(component, cmp, amt as u64);
            }
        }
        if let Some(val) = val.get(HIDDEN) {
            for (name, line) in val.grab_contents() {
                let component = cmp.get_from_name_h(name);
                let amt = parse(line, u64::MAX);
                obj.force_install_components(component, cmp, amt);
            }
        }
    }
    if let Some(val) = file.get(RESOURCES) {
        for (name, line) in val.grab_contents() {
            let resource = if let Some(val) = rss.find(name) {
                val
            } else {
                panic!("Couldn't find resource {:?}", name);
            };
            let amt = parse(line, u64::MAX);
            obj.resources_mut().add_res(resource, amt);
        }
    }
}
pub fn grab_location(file: &FileObject) -> Location {
    if let Some(val) = file.get(LOCATION) {
        Location::new(parse_field(val, f64::MAX, LOCX), parse_field(val, f64::MAX, LOCY))
    } else {
        panic!("Couldn't find location of {:?}!", file);
    }
}
pub fn rss(path: &str) -> Result<ResourceDict, String> {
    let file = format!("{}\\{}", path, RESOURCES);
    let readable: ReadableResourceDict = crate::extra_bits::result_compat(serde_json::from_str(&read_basic(&file)), |x| format!("{:?}", x))?;
    crate::extra_bits::to_result(readable.to_usable(), "Couldn't convert resourcedict into a usable format!".to_string())
}
pub fn cmp(rss: &ResourceDict, path: &str) -> Result<Components, String> {
    let file = format!("{}\\{}", path, RESOURCES);
    let readable: ReadableComponents = crate::extra_bits::result_compat(serde_json::from_str(&read_basic(&file)), |x| format!("{:?}", x))?;
    crate::extra_bits::to_result(readable.convert(rss), "Couldn't convert resourcedict into a usable format!".to_string())
}
pub fn rss_old(file: &FileObject) -> ResourceDict {
    let res = file.get(RESOURCES);
    let mut names: Vec<String> = Vec::new();
    let mut transfer_costs: Vec<u64> = Vec::new();
    if let Some(val) = res {
        for (name, data) in val.grab_contents() {
            names.push(name.clone());
            if let Some(val) = data.get(TRANSFER_COST) {
                if let Ok(val) = val.name().parse::<u64>() {
                    transfer_costs.push(val);
                } else if val.name() == "MAX" {
                    transfer_costs.push(u64::MAX);
                } else {
                    panic!("{:?} cannot be parsed!", val.name());
                }
            } else {
                panic!("Couldn't find transfer cost for {:?}!", name);
            }
        }
    } else {
        let _ = 1;
        panic!("No resource object was found in {:?}!", file);
    }
    let mut req1: HashMap<ResourceID, f64> = HashMap::new();
    let mut req2: HashMap<ResourceID, HashMap<ResourceID, f64>> = HashMap::new();
    let mut req3: Option<ResourceID> = None;
    if let Some(val) = file.get(RSSMOD) {
        rss_mod(val, &names, &mut req1, &mut req2, &mut req3);
    }
    ResourceDict::new(names, transfer_costs, req1, req2, req3)
}
pub fn rss_mod(
    file: &FileObject, names: &Vec<String>, req1: &mut HashMap<ResourceID, f64>, req2: &mut HashMap<ResourceID, HashMap<ResourceID, f64>>,
    req3: &mut Option<ResourceID>,
) {
    let mut res1: HashMap<ResourceID, f64> = HashMap::new();
    let mut res2: HashMap<ResourceID, HashMap<ResourceID, f64>> = HashMap::new();
    let mut res3: Option<ResourceID> = None;
    for (name, line) in file.grab_contents() {
        let idpos = ResourceID::new(
            names
                .iter()
                .position(|x| x == name)
                .unwrap_or_else(|| panic!("{:?} is not inside the resource dictionary!", name)),
        );
        if let Some(val) = line.get(REQUIRES) {
            let mut intermediate: HashMap<ResourceID, f64> = HashMap::new();
            for (name, new) in val.grab_contents() {
                if let Some(resource) = names.iter().position(|x| x == name) {
                    intermediate.insert(ResourceID::new(resource), parse(new, f64::MAX));
                } else {
                    panic!("{:?} is not inside the resource dictionary!", name);
                }
            }
            res2.insert(idpos, intermediate);
        }
        if let Some(val) = line.get(GROWTH) {
            res1.insert(idpos, parse(val, f64::MAX));
        }
        if line.get(TRANSFER).is_some() {
            if res3.is_none() {
                res3 = Some(idpos);
            } else {
                panic!("Only one resource can be used as transfer currency!");
            }
        }
    }
    *req1 = res1;
    *req2 = res2;
    *req3 = res3;
}
pub fn cmp_new(rss: &ResourceDict, file: &FileObject) -> Components {
    let mut cmp = Components::new();
    let mut names: Vec<String> = Vec::new();
    let mut h_names: Vec<String> = Vec::new();
    let mut components: Vec<Component> = Vec::new();
    let mut h_components: Vec<Component> = Vec::new();
    let mut r_names: Vec<String> = Vec::new();
    let mut recipes: Vec<Recipe> = Vec::new();
    if let Some(val) = file.get(COMPONENTS) {
        if let Some(val) = val.get(ACCESSIBLE) {
            for (name, val) in val.grab_contents() {
                names.push(name.clone());
                components.push(generate_component(val, rss));
            }
        }
        if let Some(val) = val.get(HIDDEN) {
            for (name, val) in val.grab_contents() {
                h_names.push(name.clone());
                h_components.push(generate_component(val, rss));
            }
        }
    }
    if let Some(val) = file.get(RECIPE) {
        for (name, val) in val.grab_contents() {
            r_names.push(name.clone());
            let mut recipe = Recipe::new(rss.len());
            for (name, val) in val.grab_contents() {
                let resource = rss.find(name).unwrap_or_else(|| panic!("Couldn't find {} in resources!", name));
                let amt = parse(val, i64::MAX);
                recipe.cost()[resource.get()] = amt;
            }
            recipes.push(recipe);
        }
    } else {
        panic!("No recipes object found!");
    }
    cmp.add_l(names, components);
    cmp.add_h_l(h_names, h_components);
    cmp.add_r_l(r_names, recipes);
    cmp
}
pub fn generate_component(file: &FileObject, rss: &ResourceDict) -> Component {
    let mut res = Component::new(rss.len());
    if let Some(val) = file.get(COST) {
        for (name, new) in val.grab_contents() {
            if let Some(resource) = rss.find(name) {
                res.change_cost(resource, parse(new, i64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary!", name);
            }
        }
    }
    if let Some(val) = file.get(SURPLUS) {
        for (name, new) in val.grab_contents() {
            if let Some(resource) = rss.find(name) {
                res.change_surplus(resource, parse(new, i64::MAX));
            } else {
                panic!(
                    "{:?} is not inside the resource dictionary! Contents of resource dictionary: {:?}",
                    name, rss
                );
            }
        }
    }
    if let Some(val) = file.get(STORAGE) {
        for (name, new) in val.grab_contents() {
            if let Some(resource) = rss.find(name) {
                res.change_storage(resource, parse(new, u64::MAX));
            } else {
                panic!("{:?} is not inside the resource dictionary!", name);
            }
        }
    }
    res
}
fn parse<T>(obj: &FileObject, max: T) -> T
where
    T: FromStr, {
    let val = obj.name().trim();
    if let Ok(val) = val.parse::<T>() {
        val
    } else if obj.name() == "MAX" {
        max
    } else {
        panic!("{:?} cannot be parsed!", obj.name());
    }
}
fn parse_field<T>(obj: &FileObject, max: T, field: &str) -> T
where
    T: FromStr, {
    if let Some(val) = obj.get(field) {
        parse(val, max)
    } else {
        panic!("{:?} cannot be parsed!", obj.name());
    }
}
pub fn load(paths: Vec<&str>) -> FileObject {
    let mut res: FileObject = FileObject::blank(String::new()); //initializes the result
    for line in paths {
        println!("PATH: {}", line);
        let v = read_folder(line); //Reads the folder
        for line in v {
            //For every file in the folder...
            res.merge(FileObject::read_from(line, String::new(), 0)); //Merges the contents
        }
    }
    res
}
