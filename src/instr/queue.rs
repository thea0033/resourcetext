use std::usize;

use crate::{
    component::ComponentDict,
    resources::ResourceDict,
    systems::{object_id::ObjectID, Systems},
    ui::io::ansi,
};

use super::Instr;
use super::InstrRes;

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
    pub fn exe(&mut self, obj: ObjectID, sys: &mut Systems, rss: &ResourceDict, cmp: &ComponentDict) -> QueueRes {
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
    pub fn display(&self, obj: ObjectID, sys: &Systems, rss: &ResourceDict, cmp: &ComponentDict) -> Vec<String> {
        let mut res = Vec::new(); //Initializes result
        for i in 0..self.queue.len() {
            let mut temp:String = format!("{}{}: {}", self.color_instr(i), i, self.queue[i].display(obj, sys, rss, cmp));
            if let InstrRes::Fail(val) = &self.last_res {
                temp.push_str(&format!("(FAILED: {})", val));
            }
            res.push(temp);
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
#[derive(Clone, Copy)]
pub struct QueueID {
    id:usize
}
impl QueueID {
    pub fn new(id: usize) -> QueueID { QueueID {id}}
    pub fn id(&self) -> usize {
        self.id
    }
}