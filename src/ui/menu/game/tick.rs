use crate::save::Package;

impl Package {
    pub fn tick(&mut self) {
        self.sys.tick(&self.rss, &self.cmp, &mut self.dir);
    }
}
