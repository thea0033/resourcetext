use std::collections::HashMap;

use crate::{
    component::ComponentDict,
    instr::directions::Directions,
    location::Location,
    merge::Merge,
    object::readable::ReadableObject,
    resources::ResourceDict,
    systems::{system_id::SystemID, Systems},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableSystem {
    location: Location,
    objects: HashMap<String, ReadableObject>,
}
impl ReadableSystem {
    /// Imports the system into the systems object supplied. Assumes that the system has already been created.
    pub fn convert(self, sys: &mut Systems, id: SystemID, dir: &mut Directions, rss: &ResourceDict, cmp: &ComponentDict) -> Result<(), String> {
        sys.get_system_mut(id).move_to(self.location); // Changes location
        for (name, line) in self.objects {
            sys.add_made_object(id, line.convert(rss, cmp, id)?, name, dir);
        }
        Ok(())
    }
    pub fn append(&mut self, other: ReadableSystem) {
        self.objects.merge(other.objects);
        self.location = other.location;
    }
}
impl Merge for ReadableSystem {
    fn merge(&mut self, other: ReadableSystem) {
        *self = other;
    } //Merging two systems overwrites the former. Use the append function to merge them together.
}
