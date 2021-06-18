pub mod condition;
pub mod directions;
pub mod instrs;
pub mod queue;
use crate::object::Object;
use crate::save::Package;
use crate::ui::menu::config::Config;
use crate::{component::ComponentDict, systems::object_id::ObjectID};

use crate::{
    component::{ComponentID, RecipeID},
    location::Location,
    resources::ResourceDict,
    systems::{system_id::SystemID, Systems},
};

use self::condition::Condition;
use self::queue::QueueID;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Instr {
    Move(Location), //Move to a location.
    Jump(SystemID), //Jump to another system.
    Transfer(Vec<u64>, ObjectID), /* Transfer resources to another object (moves to it
                     * first) */
    Grab(Vec<u64>, ObjectID), /* Grab resources from another object (moves to it
                               * first) */
    MoveTo(ObjectID), //Moves to another object
    If(Condition, Box<Instr>, Box<Instr>), /* If the condition is true, evaluates the first
                       * condition. Otherwise, evaluates the second
                       * condition. */
    All(Vec<Instr>),                      //Does all of these, in order, until a failure or delay.
    GoTo(InstrID),                        //Moves to another position on the queue.
    PerformRecipe(RecipeID, usize),       //Performs a recipe a certain number of times.
    InstallComponent(ComponentID, usize), //Installs a component a certain number of times.
    RemoveComponent(ComponentID, usize),  //Removes a component a certain number of times.
    Sticky,                               //Sticks here, doing nothign forever.
    End,                                  //Immediately goes to the next instruction.
    Fail,                                 //Fails.
} //An instruction. Automates the boring parts of this game.
pub const ALL_INSTRS: &[&str] = &[
    "Move (location): Spends movement to move to a certain location.",
    "Jump (system): Jumps to another system.",
    "Transfer (resources, object): Transfers resources to another object when they're in the same location. If not, does MoveTo.",
    "Grab (resources, object): Grabs resources from another object when they're in the same location. If not, does MoveTo. ",
    "MoveTo (object): Does jump to the system they other object is in. If they're in the same system, moves to the object.",
    "If (condition, instr, instr): If a condition is true, does one instr. Otherwise, does the other. ",
    "All (list of instrs): Does all of the instructions listed until stuck. ",
    "Go to (id): Changes the current instruction to this id. ",
    "Perform Recipe (Recipe, amount): Attempts to perform a certain recipe a certain amount of times.",
    "Install Component (Recipe, amount): Attempts to install a certian component a certain amount of times.",
    "Remove Component (Recipe, amount): Attempts to install a certian component a certain amount of times.",
    "Sticky: Is always in progress. Never finishes. Never does anything.",
    "End: Immediately finishes. ",
    "Fail: Immediately fails. Will appear in red.",
];
pub fn all_instrs() -> Vec<String> {
    ALL_INSTRS.iter().map(|x| x.to_string()).collect()
}
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct InstrID {
    //Instruction identification wrapper, to make it obvious what the usize will refer to.
    id: usize,
}
pub struct InstrLocation {
    pub obj: ObjectID,
    pub queue: QueueID,
    pub id: InstrID,
    pub all: Vec<InstrID>,
}
impl InstrLocation {
    pub fn new(obj: ObjectID, queue: QueueID, id: InstrID) -> InstrLocation {
        InstrLocation {
            obj,
            queue,
            id,
            all: Vec::new(),
        }
    }
    pub fn push(&self, next: InstrID) -> InstrLocation {
        let mut all = self.all.clone();
        all.push(next);
        InstrLocation {
            obj: self.obj,
            queue: self.queue,
            id: self.id,
            all,
        }
    }
}
impl InstrID {
    pub fn new(id: usize) -> InstrID {
        InstrID { id }
    } //Creates wrapper
    pub fn get(&self) -> usize {
        self.id
    } //Simple getter
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InstrRes {
    //Instructions, when executed, return an instruction result. Here's what these results mean:
    Success(usize), //The instruction has finished, and you should go to the next thing.
    Fail(String),   //The instruction has failed, and has an error message.
    Continue,       //The instruction is still in progress, and will continue next tick.
}
impl Instr {
    //Context required: The object that is performing the instructions, the
    // position in the queue we're in, the system dictionary, the resource
    // dictionary, the component dictionary.
    //TODO: Fix the execution problem w/ flags
    pub fn exe(&self, obj: ObjectID, pos: usize, sys: &mut Systems, rss: &ResourceDict, cmp: &ComponentDict) -> InstrRes {
        match self {
            Instr::Move(val) => {
                //Movement
                if val.eq(sys.get_object_mut(obj).get_location_mut()) {
                    //If we're already at the destination...
                    return InstrRes::Success(pos + 1); //We've succeeded! onto
                                                       // the next thing!
                }
                let movement: f64 = sys.get_object_mut(obj).resources().get_curr(rss.find("Movement").unwrap()) as f64; //Amount of movement generated
                let mass: f64 = sys.get_object_mut(obj).resources().get_curr(rss.find("Mass").unwrap()) as f64; //Mass of the object
                let distance = movement / mass; //Distance travelled (this is an Aristotelian universe, where force = mass *
                                                // velocity)
                sys.get_object_mut(obj).get_location_mut().move_towards(*val, distance); //Moves towards the location
                sys.get_object_mut(obj).resources_mut().change_amt(rss.find("Movement").unwrap(), 0); //Resets the movement generated to zero
                if (*sys.get_object_mut(obj).get_location_mut()).eq(val) {
                    //If we got there...
                    return InstrRes::Success(pos + 1); //We've succeeded! Onto
                                                       // the next thing!
                }
                InstrRes::Continue //We haven't gotten there yet!
            }
            Instr::GoTo(val) => {
                InstrRes::Success(val.id) //We've succeeded, and we're
                                          // going to the location
                                          // specified by the
                                          // instruction!
            }
            Instr::If(val1, val2, val3) => {
                if val1.eval(sys) {
                    //Evaluate the condition! If it's true...
                    val2.exe(obj, pos, sys, rss, cmp) //Execute the
                                                      // first instruction
                                                      // and return the
                                                      // result.
                } else {
                    //Otherwise...
                    val3.exe(obj, pos, sys, rss, cmp) //Execute the
                                                      // second instruction
                                                      // and return the
                                                      // result.
                }
            }
            Instr::All(val) => {
                let mut saved_pos: usize = pos + 1;
                for instr in val {
                    //For every instruction...
                    match instr.exe(obj, pos, sys, rss, cmp) {
                        //Execute the instruction.
                        InstrRes::Fail(val) => return InstrRes::Fail(val), /* If it fails, return the result. */
                        InstrRes::Continue => return InstrRes::Continue,   /* If it's incomplete, */
                        // return the result
                        // generated.
                        InstrRes::Success(val) => {
                            saved_pos = val;
                        } //If we've succeeded, store the value.
                    }
                }
                InstrRes::Success(saved_pos)
                //Returns the last stored position (which makes gotos work);
                // eg: 0. Install a component
                //1. move to (0, 0)
                //2. move to (3, 3)
                //3. go to position 1
                //This would give us a turn doing nothing.
                //Returning the last value allows us to do this:
                //0. Install a component
                //1. move to (0, 0)
                //2. do all these: [move to (3, 3), AND go to position 1]
                //No turn is wasted here.
                //NOTE: You could also install the component manually and then
                // do this: 0. move to (0, 0)
                //1. move to (3, 3)
                //After position 0, we automatically go to position 1. After
                // executing 1, we automatically go to 2, which rounds back to
                // 0.
            }
            Instr::Jump(val) => {
                //Jumps to a different system:
                if val.get() == sys.get_objects_system(obj).get() {
                    //If we're already in the right system...
                    return InstrRes::Success(pos + 1); //Success!
                }
                InstrRes::Fail("Jumping to another system hasn't been implemented yet!".to_string())
                //Jumping isn't implemented yet.
            }
            Instr::MoveTo(val) => {
                //Moves to another object.
                match Instr::Jump(sys.get_objects_system(*val)).exe(obj, pos, sys, rss, cmp) {
                    //Starts by jumping to the system the object is in.
                    InstrRes::Continue => return InstrRes::Continue, /* If it's in progress, */
                    // return the result and
                    // wait.
                    InstrRes::Fail(val) => return InstrRes::Fail(val), /* If it failed, we */
                    // return the same
                    // failure.
                    InstrRes::Success(_) => {} //If we succeeded, continue on.
                }
                Instr::Move(*sys.get_object_mut(*val).get_location_mut()).exe(obj, pos, sys, rss, cmp)
                //We move to the object's location.
            }
            Instr::Transfer(val1, val2) => {
                //Transfers resources to another object.
                let res = Instr::MoveTo(*val2).exe(obj, pos, sys, rss, cmp); //Moves to the object.
                match res {
                    InstrRes::Fail(val) => return InstrRes::Fail(val), //If we fail, fail.
                    InstrRes::Success(_) => {}                         /* If we succeed,
                                                                         * continue on in the
                                                                         * function. */
                    InstrRes::Continue => return InstrRes::Continue, /* If we aren't done, continue moving instead. */
                }
                let mut temp = rss.get_transfer_costs().iter(); //Generates transfer cost.
                let transfer_cap_cost: u64 = val1.iter().map(|x| x * temp.next().unwrap()).sum(); //Sums transfer costs up.
                let mut total_cost = val1.clone(); //Generates a clone, that we can manipulate.
                if let Some(val) = rss.get_transfer() {
                    total_cost[val.get()] += transfer_cap_cost; //Adds the cost of transferring resources on.
                }
                if !sys.get_object_mut(obj).resources_mut().spend_unsigned(&total_cost) {
                    //Attempts to spend the resources. If it fails...
                    return InstrRes::Fail("Not enough resources!".to_string()); //fail!
                }
                sys.get_object_mut(*val2).resources_mut().gain_unsigned(val1); //Otherwise, gain extra resources.
                InstrRes::Success(pos + 1) //We've succeeded!
            }
            Instr::Grab(val1, val2) => {
                //Transfers resources to another object.
                let res = Instr::MoveTo(*val2).exe(obj, pos, sys, rss, cmp); //Moves to the object.
                match res {
                    InstrRes::Fail(val) => return InstrRes::Fail(val), //If we fail, fail.
                    InstrRes::Success(_) => {}                         /* If we succeed,
                                                                         * continue on in the
                                                                         * function. */
                    InstrRes::Continue => return InstrRes::Continue, /* If we aren't done, continue moving instead. */
                }
                let mut temp = rss.get_transfer_costs().iter(); //Generates transfer cost.
                let transfer_cap_cost: u64 = val1.iter().map(|x| x * temp.next().unwrap()).sum(); //Sums transfer costs up.
                let mut total_cost = val1.clone(); //Generates a clone, that we can manipulate.
                if let Some(val) = rss.get_transfer() {
                    total_cost[val.get()] += transfer_cap_cost; //Adds the cost of transferring resources on.
                }
                if !sys.get_object_mut(*val2).resources_mut().spend_unsigned(&total_cost) {
                    //Attempts to spend the resources. If it fails...
                    return InstrRes::Fail("Not enough resources!".to_string()); //fail!
                }
                sys.get_object_mut(obj).resources_mut().gain_unsigned(val1); //Otherwise, gain extra resources.
                InstrRes::Success(pos + 1) //We've succeeded!
            }
            Instr::Sticky => InstrRes::Continue,      //Sticks to the instruction
            Instr::End => InstrRes::Success(pos + 1), //immediately advances
            Instr::Fail => InstrRes::Fail("This instruction was supposed to fail.".to_string()), /* Fails */
            Instr::PerformRecipe(recipe, amt) => {
                //Performs a recipe.
                let amt_success = sys.get_object_mut(obj).do_recipes(*recipe, cmp, *amt); //Performs recipes, gets amount of successes.
                if &amt_success == amt {
                    //If we did all of them...
                    InstrRes::Success(pos + 1) //We've succeeded!
                } else {
                    InstrRes::Fail(format!("We only had enough resources to do {} out of {} recipes", amt_success, amt))
                    //We've failed.
                }
            }
            Instr::InstallComponent(component, amt) => {
                //Installs a component.
                let amt_success = sys.get_object_mut(obj).install_components(*component, cmp, *amt); //Installs components, gets amount of successes.
                if &amt_success == amt {
                    //If we did all of them...
                    InstrRes::Success(pos + 1) //We've succeeded!
                } else {
                    InstrRes::Fail(format!(
                        "We only had enough resources to install {} out of {} components",
                        amt_success, amt
                    )) //We've failed.
                }
            }
            Instr::RemoveComponent(component, amt) => {
                //Installs a component.
                let amt_success = sys.get_object_mut(obj).remove_components(*component, cmp, *amt); //Installs components, gets amount of successes.
                if &amt_success == amt {
                    //If we did all of them...
                    InstrRes::Success(pos + 1) //We've succeeded!
                } else {
                    InstrRes::Fail(format!(
                        "We only had enough resources to install {} out of {} components",
                        amt_success, amt
                    )) //We've failed.
                }
            }
        }
    } //Executes instructions.
    pub fn display(&self, obj: ObjectID, pkg: &Package) -> String {
        match self {
            Instr::All(val) => {
                let mut res: String = "Do all: [".to_string();
                for line in val {
                    res.push_str(&line.display(obj, pkg));
                    res.push_str(", ");
                }
                res.pop();
                res.push(']');
                res
            }
            Instr::Move(val) => {
                format!(
                    "Move from ({}, {}) to ({}, {})",
                    pkg.sys.get_object(obj).get_location().x,
                    pkg.sys.get_object(obj).get_location().y,
                    val.x,
                    val.y
                )
            }
            Instr::Jump(val) => {
                format!("Jumping from {} to {}", pkg.sys.get_objects_system(obj).get(), val.get())
            }
            Instr::Transfer(val1, val2) => {
                format!(
                    "Transfer {} to {}",
                    crate::resources::display_vec_one(&pkg.rss, val1, ", "),
                    pkg.sys.get_object_name(*val2)
                )
            }
            Instr::Grab(val1, val2) => {
                format!(
                    "Grab {} from {}",
                    crate::resources::display_vec_one(&pkg.rss, val1, ", "),
                    pkg.sys.get_object_name(*val2)
                )
            }
            Instr::MoveTo(val) => {
                format!("Move to {}", pkg.sys.get_object_name(*val))
            }
            Instr::If(val1, val2, val3) => {
                format!(
                    "If [{}], then [{}] else [{}]",
                    val1.display(),
                    val2.display(obj, pkg),
                    val3.display(obj, pkg)
                )
            }
            Instr::GoTo(val) => {
                format!("Jump to instruction {}", val.get())
            }
            Instr::PerformRecipe(val1, val2) => {
                format!("Perform recipe {} {} times", pkg.cmp.get_r_name(*val1), val2)
            }
            Instr::InstallComponent(val1, val2) => {
                format!("Installing component {} {} times", pkg.cmp.get_name(*val1), val2)
            }
            Instr::RemoveComponent(val1, val2) => {
                format!("Removing component {} {} times", pkg.cmp.get_name(*val1), val2)
            }
            Instr::Sticky => "Remain here".to_string(),
            Instr::End => "Advance".to_string(),
            Instr::Fail => "Fail".to_string(),
        }
    } //Displays instructions. Shouls be simple enough.

    pub fn display_options(&self, obj: ObjectID, pkg: &Package) -> Vec<String> {
        match self {
            Instr::All(val) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Add an instruction to this:"));
                res.push(format!("Remove an instruction from this:"));
                for item in val {
                    res.push(format!("{}", item.display(obj, pkg)));
                }
                res
            }
            Instr::Move(_) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change destination"));
                res
            }
            Instr::Jump(_) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change system to jump to"));
                res
            }
            Instr::Transfer(_, _) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change resources transferred"));
                res.push(format!("Change object destination"));
                res
            }
            Instr::Grab(_, _) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change resources grabbed"));
                res.push(format!("Change object destination"));
                res
            }
            Instr::MoveTo(_) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change object destination"));
                res
            }
            Instr::If(_, _, _) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change condition"));
                res.push(format!("Change instruction if true"));
                res.push(format!("Change instruction if false"));
                res
            }
            Instr::GoTo(_) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Change id that is went to"));
                res
            }
            Instr::PerformRecipe(_, _) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Replace recipe performed"));
                res.push(format!("Replace amount of times recipe is performed"));
                res
            }
            Instr::InstallComponent(_, _) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Replace component installed"));
                res.push(format!("Replace amount of times component is installed"));
                res
            }
            Instr::RemoveComponent(_, _) => {
                let mut res: Vec<String> = Vec::new();
                res.push(format!("Replace component removed"));
                res.push(format!("Replace amount of times component is removed"));
                res
            }
            Instr::Sticky => Vec::new(),
            Instr::End => Vec::new(),
            Instr::Fail => Vec::new(),
        }
    } //Displays instruction options based on instruction
}
pub fn parse_options(input: usize, pkg: &mut Package, config: &mut Config, loc: InstrLocation) {
    match pkg.dir.get_from_loc(&loc) {
        Instr::All(val) => {
            match input {
                0 => {
                    let add_num = config.buffer.get_valid_flush(
                        "Enter the position where you want to insert your position:",
                        "Please enter a valid number!",
                        |x| *x <= val.len(),
                    ); //Inserts
                    let add_val = pkg.new_instr(config); //Gets from input
                    if let Some(v) = add_val {
                        if let Instr::All(val) = pkg.dir.get_from_loc_mut(&loc) {
                            val.insert(add_num, v);
                        } //Inserts an instruction based on the input
                    }
                }
                1 => {
                    let rmv_num: usize =
                        config
                            .buffer
                            .get_valid_flush("Enter the position you want to remove:", "Please enter a valid number", |x| {
                                *x < val.len()
                            }); //Gets from input
                    if let Instr::All(val) = pkg.dir.get_from_loc_mut(&loc) {
                        val.remove(rmv_num); //Removes the instruction
                    }
                }
                _ => {
                    pkg.instr_menu(config, loc.push(InstrID::new(input)))
                    //Enters the instruction displayed
                }
            }
        }
        Instr::Move(_) => {
            let temp = pkg.select_location(config); //Updates location
            if let Instr::Move(val) = pkg.dir.get_from_loc_mut(&loc) {
                *val = temp;
            }
        }
        Instr::Jump(_) => {
            if let Some(s) = pkg.select_system(config) {
                if let Instr::Jump(val) = pkg.dir.get_from_loc_mut(&loc) {
                    *val = s;
                }
            }
        }
        Instr::Transfer(val, _) => {
            match input {
                0 => {
                    let temp = val.clone();
                    let temp = pkg.select_resources(config, Some(temp)); //Updates resources to be transferred
                    if let Instr::Transfer(val, _) = pkg.dir.get_from_loc_mut(&loc) {
                        *val = temp;
                    }
                }
                1 => {
                    if let Some(sys) = pkg.select_system(config) {
                        if let Some(obj) = pkg.select_object(config, sys) {
                            if let Instr::Transfer(_, val) = pkg.dir.get_from_loc_mut(&loc) {
                                *val = obj;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Instr::Grab(val, _) => {
            match input {
                0 => {
                    let temp = Some(val.clone());
                    let temp = pkg.select_resources(config, temp);
                    if let Instr::Grab(val, _) = pkg.dir.get_from_loc_mut(&loc) {
                        *val = temp;
                    }
                }
                1 => {
                    if let Some(sys) = pkg.select_system(config) {
                        if let Some(obj) = pkg.select_object(config, sys) {
                            if let Instr::Grab(_, val) = pkg.dir.get_from_loc_mut(&loc) {
                                *val = obj;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Instr::MoveTo(_) => {
            if let Some(sys) = pkg.select_system(config) {
                if let Some(obj) = pkg.select_object(config, sys) {
                    if let Instr::Grab(_, val) = pkg.dir.get_from_loc_mut(&loc) {
                        *val = obj;
                    }
                }
            }
        }
        Instr::If(_, _, _) => {
            panic!("Not implemented yet!");
        }
        Instr::GoTo(val) => {
            *val = InstrID::new(config.buffer.get_flush("Enter the new position", "Please enter a valid input!"));
            //Changes the instruction we go to
        }
        Instr::PerformRecipe(recipe, amt) => {
            match input {
                0 => {
                    if let Some(val) = pkg.select_recipe(config) {
                        *recipe = val; //Option 1: Select a new recipe
                    };
                }
                1 => {
                    *amt = config.buffer.get_flush("Enter the new amount", "Please enter a valid input!");
                    //Option 2: Select a new
                    // amount
                }
                _ => {}
            }
        }
        Instr::InstallComponent(component, amt) => {
            match input {
                0 => {
                    if let Some(val) = pkg.select_component(config) {
                        *component = val; //Option 1: select a new component
                    };
                }
                1 => {
                    *amt = config.buffer.get_flush("Enter the new amount", "Please enter a valid input");
                    //Option 2: Select a new
                    // amount
                }
                _ => {}
            }
        }
        Instr::RemoveComponent(component, amt) => {
            match input {
                0 => {
                    if let Some(val) = pkg.select_component(config) {
                        *component = val; //Option 1: select a new component
                    };
                }
                1 => {
                    *amt = config.buffer.get_flush("Enter the new amount", "Please enter a valid input!");
                    //Option 2: Select a new
                    // amount
                }
                _ => {}
            }
        }
        Instr::Sticky => {} //No extra options
        Instr::End => {}    //No extra options
        Instr::Fail => {}   //No extra options
    }
}
