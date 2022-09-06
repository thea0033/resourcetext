use crate::{
    component::{readable::ReadableComponentDict, ComponentDict},
    instr::directions::Directions,
    merge::Merge,
    resources::readable::ReadableResourceDict,
    systems::{readable::ReadableSystems, Systems},
};

use super::Package;

pub struct ReadablePackage {
    pub rss: ReadableResourceDict,
    pub cmp: ReadableComponentDict,
    pub sys: ReadableSystems,
}
impl ReadablePackage {
    pub fn new(rss: ReadableResourceDict, cmp: ReadableComponentDict, sys: ReadableSystems) -> Self {
        Self { rss, cmp, sys }
    }
    pub fn default() -> Self {
        Self {
            rss:ReadableResourceDict::default(),
            cmp: ReadableComponentDict::default(),
            sys: ReadableSystems::default(),
        }
    }
}
impl Merge for ReadablePackage {
    fn merge(&mut self, other: Self) {
        self.rss.merge(other.rss);
        self.cmp.merge(other.cmp);
        self.sys.merge(other.sys);
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SemiReadablePackage {
    rss: ReadableResourceDict,
    cmp: ComponentDict,
    sys: Systems,
    dir: Directions,
}
impl SemiReadablePackage {
    pub fn from(other: &Package) -> Self {
        Self {
            rss: other.rss.to_readable(),
            cmp: other.cmp.clone(),
            sys: other.sys.clone(),
            dir: other.dir.clone(),
        }
    }
}
