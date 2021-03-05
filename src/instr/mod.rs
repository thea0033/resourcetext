pub mod condition;
use crate::resources::*;
use crate::systems::*;
use crate::{
    component::{ComponentID, Components, RecipeID},
    location::*,
    systems::{object_id::ObjectID, system_id::SystemID},
    ui::io::ansi,
};
pub mod instr;
use instr::*;

use crate::instr::condition::Condition;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Queue {
    queue: Vec<Instr>,      //The instructions themselves.
    delete_after_exe: bool, //Whether we delete instructions after they execute.
    curr: usize,            //The current instruction.
    last: usize,            //The previous instruction.
    last_res: InstrRes,     //The previous result.
    flag: Option<usize>,    //A flag.
} //A queue of instructions.
#[derive(Debug, Clone)]
pub enum QueueRes {
    Completion,   //This queue has finished, and is ready to be deleted.
    Fail(String), //This queue has failed.
    Continue,     //This queue is in progress.
} //The result of a queue's execution.
impl Queue {
    pub fn exe(&mut self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &Components) -> QueueRes {
        if let Some(mut new) = self.flag {
            //If this flag has triggered...
            self.flag = None; //Reset the flag.
            if self.delete_after_exe {
                //If we're deleting after executing...
                self.queue.remove(self.curr); //Removes the current instruction.
                if new > self.curr {
                    new -= 1; //Decreases the value in the flag by 1 to
                              // compensate for the removed instruction.
                }
            }
            self.curr = new; //Resets the current position.
        }
        if self.queue.is_empty() {
            //If the queue's length is zero...
            return QueueRes::Completion; //The queue is done.
        }
        self.curr %= self.queue.len(); //Rounds off any invalid positions

        let res = self.queue[self.curr].exe(obj, self.curr, sys, rss, cmp); //Performs the instruction at the current location.
        self.last_res = res.clone(); //Sets the last result variable.
        let placeholder = self.curr; //Sets a placeholder.
        let ret_val = match res {
            InstrRes::Success(new) => {
                self.flag = Some(new); //If we've succeeded in an instruction, we place a flag down.
                QueueRes::Continue //The queue isn't done yet.
            }
            InstrRes::Continue => QueueRes::Continue, /* If the instruction isn't done, the */
            // queue isn't done.
            InstrRes::Fail(val) => QueueRes::Fail(val), /*If the instruction fails, the queue
                                                         * fails. */
        };
        if placeholder != self.curr {
            //If the current location changed...
            self.last = placeholder; //Sets the last location to the
                                     // placeholder.
        }
        ret_val //returns the value we should return.
    }
    pub fn new(delete_after_exe: bool, first_instr: Instr) -> Queue {
        Queue {
            delete_after_exe,
            curr: 0,
            last: 0,
            queue: vec![first_instr],
            last_res: InstrRes::Continue,
            flag: None,
        }
    } //Creates a new queue.
    pub fn ins(&mut self, instr: Instr, pos: usize) {
        self.queue.insert(pos, instr);
    } //Adds a new instruction to the queue.
    pub fn rmv(&mut self, pos: usize) {
        self.queue.remove(pos);
    } //Removes an instruction from the queue.
    pub fn color(&self) -> &str {
        match self.last_res {
            InstrRes::Continue => ansi::BLUE,
            InstrRes::Fail(_) => ansi::RED,
            InstrRes::Success(_) => ansi::GREEN,
        }
    } //Returns the color of the queue (used to help the user tell which queues have
      // failed and which haven't)
    pub fn display(&self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &Components) -> String {
        let mut res = "".to_string(); //Initializes result
        for i in 0..self.queue.len() {
            res.push_str(&format!("{}{}: {}", self.color_instr(i), i, self.queue[i].display(obj, sys, rss, cmp)));
            if let InstrRes::Fail(val) = &self.last_res {
                res.push_str(&format!("(FAILED: {})\n", val));
            } else {
                res.push('\n');
            }
        }
        res
    } //Displays the queue. amt_before allows it to fit neatly
    pub fn color_instr(&self, pos: usize) -> &str {
        if pos == self.curr {
            //The current instruction is colored based on this:
            match self.last_res {
                InstrRes::Continue => {
                    return ansi::CYAN; //In-progress stuff is colored cyan.
                }
                InstrRes::Fail(_) => {
                    return ansi::RED; //Failed stuff is colored red.
                }
                InstrRes::Success(_) => {
                    return ansi::GREEN; //Succeeded stuff is colored green.
                }
            }
        }
        if pos == self.last {
            //The last instruction is colored yellow.
            return ansi::YELLOW;
        }
        ansi::RESET //All other instructions are colored white.
    } //Returns the color of the instruction.
    pub fn len(&self) -> usize {
        self.queue.len()
    } //Returns the queue's length.
    pub fn get(&mut self, pos: usize) -> &mut Instr {
        &mut self.queue[pos]
    } //Returns the instruction at the position given.
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Instrs {
    instrs: Vec<Queue>, //The queues.
    names: Vec<String>, //The names of the queues.
} //A vector of queues, basically.
impl Instrs {
    pub fn exe(&mut self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &Components) {
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
                self.rmv(i); //Removes the queue.
            }
        }
    }
    pub fn new() -> Instrs {
        Instrs {
            instrs: Vec::new(),
            names: Vec::new(),
        }
    }
    pub fn add(&mut self, queue: Queue, name: String) {
        self.instrs.push(queue);
        self.names.push(name);
    } //Adds a queue and name
    pub fn rmv(&mut self, index: usize) {
        self.instrs.remove(index);
        self.names.remove(index);
    } //Removes a queue and name
    pub fn get_queue(&mut self, id: usize) -> &mut Queue {
        &mut self.instrs[id]
    } //Gets the queue based on the position
    pub fn get_queues(&self) -> &Vec<Queue> {
        &self.instrs
    } //Gets the queue based on the position
    pub fn len(&self) -> usize {
        self.instrs.len()
    } //Gets the length.
    pub fn display(&self) -> String {
        let mut res = "".to_string();
        for i in 0..self.instrs.len() {
            res.push_str(&format!("{}{}: {}\n", self.instrs[i].color(), i, self.names[i]));
        }
        res
    } //Displays the object.
    pub fn get_name(&self, id: usize) -> String {
        self.names[id].clone()
    } //Gets the name.
    pub fn merge(&mut self, other: &Instrs) {
        for i in 0..other.len() {
            self.add(other.instrs[i].clone(), other.names[i].clone());
        }
    } //Merges the other instructions into this one
}
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
    pub fn exe(&mut self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &Components) {
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
    pub fn display(&self, amt_before: usize, obj: ObjectID, sys: &Systems, rss: &ResourceDict, cmp: &Components) -> String {
        let mut res: String = "".to_string(); //Initializes result
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
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Directions {
    directions: Vec<Instrs>,
    quick: Vec<Quickie>,
} //Each position corresponds to an object.
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
