use crate::{
    component::readable::ReadableComponentDict, merge::Merge, resources::readable::ReadableResourceDict, systems::readable::ReadableSystems,
};

pub struct ReadablePackage {
    pub rss: ReadableResourceDict,
    pub cmp: ReadableComponentDict,
    pub sys: ReadableSystems,
}
impl ReadablePackage {
    pub fn new(rss: ReadableResourceDict, cmp: ReadableComponentDict, sys: ReadableSystems) -> Self {
        Self { rss, cmp, sys }
    }
}
impl Merge for ReadablePackage {
    fn merge(&mut self, other: Self) {
        self.rss.merge(other.rss);
        self.cmp.merge(other.cmp);
        self.sys.merge(other.sys);
    }
}
