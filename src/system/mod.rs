pub mod readable;

use crate::{
    location::Location,
    systems::{object_id::ObjectID, Systems},
    ui::io::ansi,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    name: String,        //The system's name
    loc: Location,       //Where the system is located
    objs: Vec<ObjectID>, //The objects the system contains
} //A (star) system.
impl System {
    pub fn new(name: String, loc: Location) -> System {
        System { name, loc, objs: Vec::new() }
    } //Basic constructor
    pub fn move_to(&mut self, loc: Location) {
        self.loc = loc;
    }
    pub fn add_obj(&mut self, id: ObjectID) {
        self.objs.push(id);
    } //Adds an object to the system
    pub fn tick(&self) {} //Tick function; currently a placeholder but might do something someday
    pub fn len(&self) -> usize {
        self.objs.len()
    } //The number of objects in the system
    pub fn display(&self, names: &Vec<String>, sys: &Systems) -> String {
        let mut res: String = "".to_string();
        for i in 0..self.objs.len() {
            res.push_str(sys.get_object(self.objs[i]).color());
            res.push_str(&format!("{}. {}\n", i, names[self.objs[i].get()]));
        }
        res
    } //Basic display function
    pub fn display_filtered(&self, amt_before: usize, will_display: &Vec<bool>, names: &Vec<String>) -> String {
        let mut res: String = "".to_string();
        let mut i: usize = 0;
        for d in will_display {
            if *d {
                //If we want to display this...
                res.push_str(&format!("{}: {}\n", i + amt_before, names[self.objs[i].get()])); //Display it
                i += 1; //Increment the counter
            }
        }
        res
    } //Filtered display function
    pub fn get_objs(&self) -> &Vec<ObjectID> {
        &self.objs
    } //Getter
    pub fn color(&self, sys: &Systems) -> &str {
        let mut is_yellow: bool = true;
        let mut is_green: bool = false;
        for i in &self.objs {
            let c = sys.get_object(*i).color();
            if c == ansi::RED {
                return ansi::RED;
            } //Returns red if an object is red
            if c != ansi::YELLOW {
                is_yellow = false;
            } //Marks that there isn't a yellow object right now
            if c == ansi::GREEN {
                is_green = true;
            } //Marks that there is a green object
        }
        if is_yellow {
            return ansi::YELLOW; //If it's all yellow, returns yellow
        }
        if is_green {
            return ansi::GREEN; //If it isn't all yellow and there's a green
                                // object, returns green
        }
        ansi::BLUE //Otherwise, returns blue
    } //Returns the color of the system
}
