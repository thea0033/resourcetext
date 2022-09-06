use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemID {
    id: usize,
} //Wrapper class for system pointer
impl SystemID {
    pub fn new(id: usize) -> SystemID {
        SystemID { id }
    }
    pub fn get(&self) -> usize {
        self.id
    }
}
