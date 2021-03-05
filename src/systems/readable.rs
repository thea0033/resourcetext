use std::collections::HashMap;

use crate::{component::Components, instr::Directions, location::Location, merge::Merge, resources::ResourceDict, system::{System, readable::ReadableSystem}};

use super::{Systems, system_id::SystemID};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableSystems {
    systems: HashMap<String, ReadableSystem>,
}
impl ReadableSystems {
    pub fn convert(self, rss: &ResourceDict, cmp: &Components, dir: &mut Directions) -> Result<Systems, String> {
        let mut s = Systems::new();
        for (i, (name, line)) in self.systems.into_iter().enumerate() {
            s.add_system(name, Location::new(0.0, 0.0));
            line.convert(&mut s, SystemID::new(i), dir, rss, cmp)?;
        }
        Ok(s)
    }
}
impl Merge for ReadableSystems {
    fn merge(&mut self, mut other: Self) {
        self.systems.merge(other.systems);
    }
}