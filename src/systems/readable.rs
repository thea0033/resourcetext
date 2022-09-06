use std::collections::HashMap;

use crate::{
    component::ComponentDict, instr::directions::Directions, location::Location, merge::Merge, resources::ResourceDict,
    system::readable::ReadableSystem,
};

use super::{system_id::SystemID, Systems};
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableSystems {
    systems: HashMap<String, ReadableSystem>,
}
impl ReadableSystems {
    pub fn convert(self, rss: &ResourceDict, cmp: &ComponentDict, dir: &mut Directions) -> Result<Systems, String> {
        let mut s = Systems::new();
        for (i, (name, line)) in self.systems.into_iter().enumerate() {
            s.add_system(name, Location::new(0.0, 0.0));
            line.convert(&mut s, SystemID::new(i), dir, rss, cmp)?;
        }
        Ok(s)
    }
    pub fn default() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }
}
impl Merge for ReadableSystems {
    fn merge(&mut self, other: Self) {
        self.systems.merge(other.systems);
    }
}
