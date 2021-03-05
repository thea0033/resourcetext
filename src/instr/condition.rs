use crate::{
    resources::Resources,
    systems::{object_id::ObjectID, Systems},
};
//Conditions - used for advanced logic in-game. Not implemented yet!
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Condition {
    _Not(Box<Condition>),      //Basic logic
    _And(Vec<Condition>),      //True if all are true, false otherwise
    _Or(Vec<Condition>),       //False if all are false, true otherwise
    _Has(ObjectID, Resources), //True if the object has certain resources, false otherwise
}
impl Condition {
    pub fn eval(&self, _sys: &Systems) -> bool {
        panic!("Not implemented yet!");
    } //Will evaluate a condition
    pub fn display(&self) -> String {
        panic!("Not implemented yet!");
    } //Will display a condition
}
