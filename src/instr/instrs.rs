use crate::{
    component::ComponentDict,
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
};

use super::queue::{Queue, QueueID, QueueRes};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Instrs {
    instrs: Vec<Queue>, //The queues.
    names: Vec<String>, //The names of the queues.
} //A vector of queues, basically.
impl Instrs {
    pub fn exe(&mut self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &ComponentDict) {
        let mut will_remove: Vec<bool> = Vec::new(); //Whether we should remove the queues.
        for instr in &mut self.instrs {
            //For every queue...
            if let QueueRes::Completion = instr.exe(obj, sys, rss, cmp) {
                //Executes the queues. If they're complete...
                will_remove.push(true); //Markes them to be deleted.
            } else {
                will_remove.push(false); //Otherwise, marks them not to be
                                         // deleted.
            }
        }
        let mut i = will_remove.len();
        while i > 0 {
            //Deconstructs the vector in reverse order.
            i -= 1;
            if will_remove[i] {
                //If marked for removal...
                self.rmv(QueueID::new(i)); //Removes the queue.
            }
        }
    }
    pub fn new() -> Instrs {
        Instrs {
            instrs: Vec::new(),
            names: Vec::new(),
        }
    }
    pub fn insert(&mut self, pos: QueueID, queue: Queue, name: String) {
        self.instrs.insert(pos.id(), queue);
        self.names.insert(pos.id(), name);
    } //Adds a queue and name
    pub fn add(&mut self, queue: Queue, name: String) {
        self.instrs.push(queue);
        self.names.push(name);
    }
    pub fn rmv(&mut self, index: QueueID) {
        self.instrs.remove(index.id());
        self.names.remove(index.id());
    } //Removes a queue and name
    pub fn queue_mut(&mut self, id: QueueID) -> &mut Queue {
        &mut self.instrs[id.id()]
    } //Gets the queue based on the position
    pub fn queue(&self, id: QueueID) -> &Queue {
        &self.instrs[id.id()]
    } //Gets the queue based on the position
    pub fn queues(&self) -> &Vec<Queue> {
        &self.instrs
    } //Gets the queue based on the position
    pub fn len(&self) -> usize {
        self.instrs.len()
    } //Gets the length.
    pub fn display(&self) -> Vec<String> {
        let mut res = Vec::new();
        for i in 0..self.instrs.len() {
            res.push(format!("{}{}\n", self.instrs[i].color(), self.names[i]));
        }
        res
    } //Displays the object.
    pub fn get_name(&self, id: QueueID) -> String {
        self.names[id.id()].clone()
    } //Gets the name.
    pub fn merge(&mut self, other: &Instrs) {
        for i in 0..other.len() {
            self.add(other.instrs[i].clone(), other.names[i].clone());
        }
    } //Merges the other instructions into this one
}
