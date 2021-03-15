use crate::{
    component::ComponentDict,
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
    ui::io::ansi,
};

use super::{Instr, InstrRes};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Quickie {
    dirs: Vec<Instr>,   //The directions
    res: Vec<InstrRes>, //The last results of the instruction
    del: Vec<bool>,     //Whether these will be deleted after they're finished or failed.
} //A quick instruction storage. Made out of instructions, not queues, for easy
  // access.
impl Quickie {
    pub fn new() -> Quickie {
        Quickie {
            dirs: Vec::new(),
            res: Vec::new(),
            del: Vec::new(),
        }
    } //Initializes the structure
    pub fn get_dirs(&self) -> &Vec<Instr> {
        &self.dirs
    }
    pub fn get_del(&self) -> &Vec<bool> {
        &self.del
    }
    pub fn exe(&mut self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &ComponentDict) {
        let mut will_remove: Vec<bool> = Vec::new(); //Whether to remove the instructions
        for i in 0..self.dirs.len() {
            if self.del[i] {
                //If these are marked to be deleted...
                if let InstrRes::Continue = self.res[i] { //If the instruction is still going...
                } else {
                    //Otherwise...
                    will_remove.push(true); //Marks index for removal
                    continue;
                }
            }
            let new_res = self.dirs[i].exe(obj, 0, sys, rss, cmp);
            self.res[i] = new_res; //Updates the result
            will_remove.push(false);
        }
        let mut i = will_remove.len(); //See a similar process in instrs.
        while i > 0 {
            i -= 1;
            if will_remove[i] {
                self.rmv(i);
            }
        }
    } //Execution
    pub fn rmv(&mut self, index: usize) {
        self.dirs.remove(index);
        self.res.remove(index);
        self.del.remove(index);
    } //Removes a certain index
    pub fn ins(&mut self, index: usize, instr: Instr, del: bool) {
        self.dirs.insert(index, instr);
        self.res.insert(index, InstrRes::Continue);
        self.del.insert(index, del);
    } //Adds a new function
    pub fn display(&self, amt_before: usize, obj: ObjectID, sys: &Systems, rss: &ResourceDict, cmp: &ComponentDict) -> String {
        let mut res: String = String::new(); //Initializes result
        for (i, line) in self.dirs.iter().enumerate() {
            res.push_str(&format!("{}{}. {}", self.color(i), i + amt_before, line.display(obj, sys, rss, cmp))); //Adds a few things
            if self.del[i] {
                //If it's temporary...
                res.push_str(" (temp)"); //helpful text
            } else {
                res.push_str(" (perm)"); //helpful text
            }
            if let InstrRes::Fail(val) = &self.res[i] {
                res.push_str(&format!("(FAILED: {})", val)); //If it's failed,
                                                             // more helpful
                                                             // text
            }
            res.push('\n'); //newline character for formatting
        }
        res
    } //Displays the quick queue
    pub fn color(&self, i: usize) -> &str {
        match self.res[i] {
            InstrRes::Continue => ansi::BLUE,
            InstrRes::Fail(_) => ansi::RED,
            InstrRes::Success(_) => ansi::GREEN,
        }
    } //Gives it a color based on the index and result
    pub fn len(&self) -> usize {
        self.dirs.len()
    } //Returns the length
    pub fn get(&mut self, i: usize) -> &mut Instr {
        &mut self.dirs[i]
    } //Gets a certain index
}
