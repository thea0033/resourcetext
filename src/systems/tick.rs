use crate::{instr::Directions, systems::*};
impl Systems {
    pub fn tick(&mut self, rss: &ResourceDict, cmp: &Components, dir: &mut Directions) {
        for obj in &mut self.objects {
            obj.tick(rss);
        } //All objects advance a tick
        for sys in &mut self.systems {
            sys.tick();
        } //All systems advance a tick
        let mut i = 0;
        for instr in dir.quickies() {
            instr.exe(ObjectID::new(i), self, rss, cmp);
            i += 1;
        } //All quick instructions are executed
        i = 0;
        for instr in dir.directions() {
            instr.exe(ObjectID::new(i), self, rss, cmp);
            i += 1;
        } //All queue instructions are executed
    } //Ticks
}
