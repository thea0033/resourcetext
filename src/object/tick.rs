use crate::object::Object;
use crate::resources::ResourceDict;
impl Object {
    pub fn tick(&mut self, rss: &ResourceDict) {
        self.past = self.resources.clone(); //"backs up" the current resource amount
        self.resources.tick(); //does a tick of the resources
        self.grow(rss); //Growth
        self.upkeep(rss); //Upkeep
    }
    pub fn grow(&mut self, rss: &ResourceDict) {
        for (id, amt) in rss.get_growth() {
            let pops = self.resources.get_curr(*id);
            if pops == 0 {
                return;
            } // If there's no population, doing these calculations makes no sense.
            let housing = self.resources.get_cap(*id);
            let growth = (pops as f64) * (1.0 + amt * ((housing as f64 - pops as f64) / housing as f64));
            let grown = rand_round(growth, |x| x as u64);
            self.resources.change_amt(*id, grown);
        }
    }
    pub fn upkeep(&mut self, rss: &ResourceDict) -> bool {
        let mut satisfied: bool = true;
        for (id, map) in rss.get_requirements() {
            for (upkeep_id, upkeep) in map {
                let amt = rand_round(self.resources.get_curr(*id) as f64 * upkeep, |x| x as i64); //Gets the amount of upkeep required
                if amt > 0 {
                    if !self.resources.rmv_res(*upkeep_id, amt as u64) {
                        let bottleneck = self.resources.get_curr(*id);
                        self.resources.rmv_res_force(*upkeep_id, amt as u64); //Gets rid of all of the resource required for upkeep
                        let survived: f64 = bottleneck as f64 / upkeep;
                        let survived = rand_round(survived, |x| x as u64);
                        self.resources.change_amt(*id, survived); //Kills all of the things we can't support
                        satisfied = false;
                    }
                } else {
                    self.resources.add_res(*upkeep_id, (-amt) as u64);
                }
            }
        }
        satisfied
    }
}
pub fn rand_round<T, P>(a: f64, mut b: P) -> T
where
    P: FnMut(f64) -> T, {
    if a % 1.0 > rand::random() {
        b(a.ceil())
    } else {
        b(a.floor())
    }
}
