use crate::systems::object_id::ObjectID;

use super::{instrs::Instrs, quickie::Quickie};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Directions {
    directions: Vec<Instrs>,
    quick: Vec<Quickie>,
} //Each item corresponds to an object.
impl Directions {
    pub fn new() -> Directions {
        Directions {
            directions: Vec::new(),
            quick: Vec::new(),
        }
    } //Basic new function
    pub fn directions(&mut self) -> &mut Vec<Instrs> {
        &mut self.directions
    }
    ///Returns directions
    pub fn instrs(&mut self, id: ObjectID) -> &mut Instrs {
        &mut self.directions[id.get()]
    } //Returns instruction vector at a certain object id
    pub fn add_new(&mut self) {
        self.directions.push(Instrs::new());
        self.quick.push(Quickie::new());
    } //Adds a new instruction queue; corresponds with object creation
    pub fn quickie(&mut self, id: ObjectID) -> &mut Quickie {
        &mut self.quick[id.get()]
    } //Returns the corresponding quick queue
    pub fn quickies(&mut self) -> &mut Vec<Quickie> {
        &mut self.quick
    } //Returns all of the quick queues.
}
