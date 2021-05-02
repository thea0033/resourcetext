use crate::systems::object_id::ObjectID;

use super::instrs::Instrs;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Directions {
    directions: Vec<Instrs>,
} //Each item corresponds to an object.
impl Directions {
    pub fn new() -> Directions {
        Directions {
            directions: Vec::new(),
        }
    } //Basic new function
    pub fn directions(&mut self) -> &mut Vec<Instrs> {
        &mut self.directions
    }
    ///Returns directions
    pub fn instrs_mut(&mut self, id: ObjectID) -> &mut Instrs {
        &mut self.directions[id.get()]
    } //Returns instruction vector at a certain object id
    pub fn add_new(&mut self) {
        self.directions.push(Instrs::new());
    } //Adds a new instruction queue; corresponds with object creation
}
