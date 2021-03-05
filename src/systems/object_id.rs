#[derive(Clone, Debug, Copy, serde::Serialize, serde::Deserialize)]
pub struct ObjectID {
    id: usize,
} //Wrapper class for object pointer
impl ObjectID {
    pub fn new(id: usize) -> ObjectID {
        ObjectID { id }
    } //New function
    pub fn get(&self) -> usize {
        self.id
    } //Getter function
}
