use crate::systems::object_id::ObjectID;

use super::{instrs::Instrs, Instr, InstrLocation};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Directions {
    directions: Vec<Instrs>,
} //Each item corresponds to an object.
impl Directions {
    pub fn new() -> Directions {
        Directions { directions: Vec::new() }
    } //Basic new function
    pub fn directions(&mut self) -> &mut Vec<Instrs> {
        &mut self.directions
    }
    ///Returns directions
    pub fn get_mut(&mut self, id: ObjectID) -> &mut Instrs {
        &mut self.directions[id.get()]
    } //Returns instruction vector at a certain object id
    pub fn get(&self, id: ObjectID) -> &Instrs {
        &self.directions[id.get()]
    } //Returns instruction vector at a certain object id
    pub fn insert(&mut self) {
        self.directions.push(Instrs::new());
    } //Adds a new instruction queue; corresponds with object creation
    pub fn get_from_loc(&self, loc: &InstrLocation) -> &Instr {
        let mut res = self.get(loc.obj).queue(loc.queue).get(loc.id);
        for line in loc.all.clone() {
            if let Instr::All(val) = res {
                res = &val[line.get()];
            } else if let Instr::If(_, val1, val2) = res {
                match line.get() {
                    0 => res = val1,
                    1 => res = val2,
                    _ => panic!("Something went horribly wrong!"),
                }
            } else {
                panic!("Something went horribly wrong!")
            }
        }
        res
    }
    pub fn get_from_loc_mut(&mut self, loc: &InstrLocation) -> &mut Instr {
        let mut res = self.get_mut(loc.obj).queue_mut(loc.queue).get_mut(loc.id);
        for line in loc.all.clone() {
            if let Instr::All(val) = res {
                res = &mut val[line.get()];
            } else if let Instr::If(_, val1, val2) = res {
                match line.get() {
                    0 => res = val1,
                    1 => res = val2,
                    _ => panic!("Something went horribly wrong!"),
                }
            } else {
                panic!("Something went horribly wrong!")
            }
        }
        res
    }
}
