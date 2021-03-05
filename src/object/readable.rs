use std::collections::HashMap;

use crate::{component::Components, location::Location, merge::Merge, resources::{ResourceDict, readable::ReadableResources}, systems::system_id::SystemID};

use super::Object;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableObject {
    location: Location,              //The object's current location.
    resources: ReadableResources,            //The resources the object has.
    components: HashMap<String, u64>,   //Tracker for each component.
    hidden_components: HashMap<String, u64>, //Tracker for each hidden component.
    name: String,                    //The object's name.
}

impl ReadableObject {
    pub fn convert(self, rss: &ResourceDict, cmp: &Components, system: SystemID) -> Result<Object, String> {
        let mut res = Object::new(rss, cmp, self.name, self.location, system);
        for (name, amount) in self.components {
            res.force_install_components(cmp.get_from_name(&name), cmp, amount);
        }//Installs components
        for (name, amount) in self.hidden_components {
            res.force_install_components(cmp.get_from_name_h(&name), cmp, amount);
        }//Installs hidden components
        res.resources.add(&self.resources.convert(rss)?);//Adds resources (the total resources at the end is
        // the amount specified plus the amount from components. )
        Ok(res)
    }
    pub fn append(&mut self, other: ReadableObject) {
        self.location = other.location;
        self.resources.merge(other.resources);
        self.components.merge(other.components);
        self.hidden_components.merge(other.hidden_components);
        // self.name = other.name is not needed; the names will match. 
    }
}
impl Merge for ReadableObject {
    fn merge(&mut self, other: ReadableObject) {
        *self = other;
    }
}