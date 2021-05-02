use crate::{instr::directions::Directions, systems::*};
impl Systems {
    /// Goes through each object sequentially. At this level, one thread will do it.
    pub fn tick(&mut self, rss: &ResourceDict, cmp: &ComponentDict, dir: &mut Directions) {
        for obj in &mut self.objects {
            obj.tick(rss);
        } //All objects advance a tick

        for sys in &mut self.systems {
            sys.tick();
        } //All systems advance a tick
        for (i, instr) in dir.directions().iter_mut().enumerate() {
            instr.exe(ObjectID::new(i), self, rss, cmp);
        } //All queue instructions are executed
    } //Ticks
}
