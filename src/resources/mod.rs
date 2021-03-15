pub mod dict;
pub mod readable;

use std::{cmp, collections::HashMap, fmt::Display};

use cmp::Ordering;

use crate::ui::io::ansi;

use self::readable::{ReadableResource, ReadableResourceDict};
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Resources {
    curr: Vec<u64>,    //The amount of resources here
    surplus: Vec<i64>, //The current amount increases (or decreases) by this much each tick.
    cap: Vec<u64>,     //Every resource is reduced to its cap at the end of each tick.
} //Resources
impl Resources {
    pub fn new(len: usize) -> Resources {
        Resources {
            curr: vec![0; len],
            surplus: vec![0; len],
            cap: vec![0; len],
        }
    } //Basic new function
    pub fn tick(&mut self) -> Vec<bool> {
        let mut res: Vec<bool> = Vec::new(); //Initializes result
        for i in 0..self.curr.len() {
            //For every resource...
            if self.curr[i] > self.cap[i] {
                //If we have more resources than we have capacity for...
                self.curr[i] = self.cap[i]; //Delete the extra resources
            }
            if self.surplus[i] < 0 && self.curr[i] >= (-self.surplus[i]) as u64 {
                //If we have a negative surplus, but we can still lose a few...
                self.curr[i] -= (-self.surplus[i]) as u64; //Do it
                res.push(false); //We didn't run out of resources yet...
            } else if self.surplus[i] >= 0 {
                //If we have positive (or zero) surplus...
                self.curr[i] += self.surplus[i] as u64;
                res.push(false); //We didn't run out of resource, obviously!
            } else {
                res.push(true); //We ran out!
            }
        }
        res
    } //The tick function. For each resource, returns true if
    pub fn get_curr(&self, id: ResourceID) -> u64 {
        self.curr[id.get()]
    } //Gets the current amount of this resource
    pub fn get_currs(&self) -> &Vec<u64> {
        &self.curr
    } //Gets the current amounts of all resources
    pub fn get_cap(&self, id: ResourceID) -> u64 {
        self.cap[id.get()]
    } //Gets the current storage of this resource
    pub fn get_caps(&self) -> &Vec<u64> {
        &self.cap
    } //Gets the current amounts of all resources
    pub fn get_surplus(&self, id: ResourceID) -> i64 {
        self.surplus[id.get()]
    } //Gets the current storage of this resource
    pub fn get_surplusses(&self) -> &Vec<i64> {
        &self.surplus
    } //Gets the current amounts of all resources
    pub fn spend(&mut self, other: &Vec<i64>) -> bool {
        for (i, item) in other.iter().enumerate() {
            //For every resource...
            if (self.curr[i] as i64) < *item {
                //If we can't spend this resource...
                return false; //We can't do this operation.
            }
        }
        for (i, item) in other.iter().enumerate() {
            //Performs the operation
            if *item >= 0 {
                self.curr[i] -= *item as u64;
            } else {
                self.curr[i] += (-*item) as u64;
            }
        }
        true //We did this operation.
    } //Attempts to spend these resources. Returns true if the operation was
      // successful.
    pub fn spend_unsigned(&mut self, other: &Vec<u64>) -> bool {
        for (i, item) in other.iter().enumerate() {
            //Same logic as above
            if self.curr[i] < *item {
                return false;
            }
        }
        for (i, item) in other.iter().enumerate() {
            self.curr[i] -= *item;
        }
        true
    } //Attempts to spend these resources. Returns true if the operation was
      // successful.
    pub fn amt_contained(&self, other: &Vec<i64>) -> usize {
        let mut min = usize::MAX; //Defaults to the maximum value possible
        for (i, item) in other.iter().enumerate() {
            //For every resource...
            if *item <= 0 {
                continue; //If it doesn't cost anything, or gives you
                          // something, we skip it.
            }
            let min_amt: usize = (self.curr[i] as usize / *item as usize) as usize; //Division by zero is impossible, by the way. Calculates the number of times
                                                                                    // you can buy a component.
            if min_amt < min {
                min = min_amt; //Updates the mimimum value
            }
        }
        min //Returns the value
    } //Returns the amount of times you can spend other.
    pub fn force_spend(&mut self, other: &Vec<i64>) {
        for (i, item) in other.iter().enumerate() {
            if (self.curr[i] as i64) < *item {
                self.curr[i] = 0; //sets value to zero
            } else if *item >= 0 {
                self.curr[i] -= *item as u64;
            } else {
                self.curr[i] += -*item as u64;
            }
        }
    } //Forceful spending. Exactly the same as spending, but no check
    pub fn gain(&mut self, other: &Vec<i64>) -> bool {
        for (i, item) in other.iter().enumerate() {
            if (self.curr[i] as i64) < -item {
                return false;
            }
        }
        for (i, item) in other.iter().enumerate() {
            if other[i] >= 0 {
                self.curr[i] += *item as u64;
            } else {
                self.curr[i] -= (-item) as u64;
            }
        }
        true
    } //Like spend, but it's a negative version.
    pub fn gain_unsigned(&mut self, other: &Vec<u64>) {
        for (i, item) in other.iter().enumerate() {
            self.curr[i] += *item;
        }
    } //Gain the values inputted. They're positive, so checks aren't required.
    pub fn add_surplus_vec(&mut self, other: &Vec<i64>) {
        for (i, item) in other.iter().enumerate() {
            self.surplus[i] += item;
        }
    } //Gain the values inputted to surplus.
    pub fn add_storage_vec(&mut self, other: &Vec<u64>) {
        for (i, item) in other.iter().enumerate() {
            self.cap[i] += item;
        }
    } //Add the values inputted to storage.
    pub fn add_curr_vec(&mut self, other: &Vec<u64>) {
        for (i, item) in other.iter().enumerate() {
            println!("Adding {} to {}", self.curr[i], item);
            self.curr[i] += item;
        }
    } //Add the values inputted to current.
    pub fn add(&mut self, other: &Resources) {
        self.add_curr_vec(other.get_currs());
        self.add_storage_vec(other.get_caps());
        self.add_surplus_vec(other.get_surplusses());
    }
    pub fn rmv_surplus_vec(&mut self, other: &[i64]) {
        for (i, item) in other.iter().enumerate() {
            self.surplus[i] -= item;
        }
    } //Same as add_surplus_vec, but negative.
    pub fn rmv_storage_vec(&mut self, other: &[u64]) -> bool {
        for (i, item) in other.iter().enumerate() {
            if self.cap[i] < *item {
                return false;
            }
        }
        for (i, item) in other.iter().enumerate() {
            self.cap[i] -= item;
        }
        true
    } //Spend_unsigned, but removes values from storage instead.
    pub fn can_rmv_storage_vec(&mut self, other: &[u64]) -> bool {
        for (i, item) in other.iter().enumerate() {
            if self.cap[i] < *item {
                return false;
            }
        }
        true
    } //rmv_storage_vec, but just checks if it's possible.
    pub fn add_res(&mut self, id: ResourceID, qty: u64) {
        self.curr[id.get()] += qty;
    } //Adds a certain amount of a certain resource to the resources.
    pub fn rmv_res(&mut self, id: ResourceID, qty: u64) -> bool {
        if self.curr[id.get()] < qty {
            false
        } else {
            self.curr[id.get()] -= qty;
            true
        }
    } //Tries to remove a certain amount of a certain resource from the resources.
      // Returns true if it worked.
    pub fn rmv_res_force(&mut self, id: ResourceID, qty: u64) {
        if self.curr[id.get()] < qty {
            self.curr[id.get()] = 0;
        } else {
            self.curr[id.get()] -= qty;
        }
    } //Forcefully removes a resoure.
    pub fn display(&self, rss: &ResourceDict, prev: &Resources) -> String {
        let mut res: String = String::new();
        for i in 0..rss.len() {
            let id = ResourceID::new(i);
            if self.should_display(prev, id) {
                res.push_str(&format!(
                    "{color}{name}: {amt}/{storage} {surplus}\n",
                    color = self.get_color(prev, id),
                    name = rss.get(id),
                    amt = self.get_curr(id),
                    storage = self.get_cap_fmt(id),
                    surplus = self.get_surplus_fmt(prev, id)
                ));
            }
        }

        res
    }
    pub fn get_color(&self, prev: &Resources, id: ResourceID) -> &str {
        match self.get_curr(id).cmp(&prev.get_curr(id)) {
            Ordering::Less => ansi::RED,
            Ordering::Equal => ansi::YELLOW,
            Ordering::Greater => ansi::GREEN,
        }
    }

    pub fn get_cap_fmt(&self, id: ResourceID) -> String {
        let v = self.cap[id.get()];
        if v == u64::MAX {
            "MAX".to_string()
        } else {
            v.to_string()
        }
    } //Gets the current storage of this resource
    pub fn get_surplus_fmt(&self, prev: &Resources, id: ResourceID) -> String {
        let value = i128::from(self.get_curr(id)) - i128::from(prev.get_curr(id));
        match value.cmp(&0) {
            Ordering::Equal => "".to_string(),
            Ordering::Greater => format!("(+{})", value),
            Ordering::Less => format!("({})", value),
        }
    }
    pub fn should_display(&self, prev: &Resources, id: ResourceID) -> bool {
        self.get_curr(id) != 0 || self.get_surplus(id) != 0 || prev.get_curr(id) != 0 || prev.get_surplus(id) != 0
    }
    pub fn change_amt(&mut self, id: ResourceID, new_amt: u64) {
        self.curr[id.get()] = new_amt;
    } //Basic functions; self-explanatory
    pub fn change_cap(&mut self, id: ResourceID, new_amt: u64) {
        self.cap[id.get()] = new_amt;
    } //Basic functions; self-explanatory
    pub fn change_surplus(&mut self, id: ResourceID, new_amt: i64) {
        self.surplus[id.get()] = new_amt;
    } //Basic functions; self-explanatory
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ResourceID {
    id: usize,
} //Resource identification wrapper; to make code cleaner
impl ResourceID {
    pub const fn new(id: usize) -> ResourceID {
        ResourceID { id }
    } //basic new function
    pub fn get(&self) -> usize {
        self.id
    } //basic get function
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceDict {
    names: Vec<String>,
    transfer_costs: Vec<u64>,
    growth: HashMap<ResourceID, f64>,
    requirements: HashMap<ResourceID, HashMap<ResourceID, f64>>,
    transfer_resource: Option<ResourceID>,
    //Growth:
} //Resource dictionary; contains helpful informations
impl ResourceDict {
    pub fn to_readable(&self) -> ReadableResourceDict {
        let temp = self.growth.iter();
        let mut growth: HashMap<String, f64> = HashMap::new();
        for (x, y) in temp {
            growth.insert(self.names[x.get()].clone(), *y);
        }
        let mut requirements: HashMap<String, HashMap<String, f64>> = HashMap::new();
        let temp = self.requirements.iter();
        for (x, y) in temp {
            let mut new_y: HashMap<String, f64> = HashMap::new();
            let temp = y.iter();
            for (x, y) in temp {
                new_y.insert(self.names[x.get()].clone(), *y);
            }
            requirements.insert(self.names[x.get()].clone(), new_y);
        }
        let mut resources: Vec<ReadableResource> = Vec::new();
        for (name, value) in self.names.iter().zip(self.transfer_costs.iter()) {
            resources.push(ReadableResource::new(name.clone(), *value));
        }
        ReadableResourceDict::new(
            resources,
            growth,
            requirements,
            match &self.transfer_resource {
                Some(val) => Some(self.names[val.get()].clone()),
                None => None,
            },
        )
    }
}
pub fn display_vec_one(rss: &ResourceDict, amts: &[u64], sep: &str) -> String {
    let mut res = String::new(); //Initializes result
    for (i, item) in amts.iter().enumerate() {
        if *item == 0 {
            continue;
        }
        res.push_str(&item.to_string()); //45
        res.push(' '); //
        res.push_str(&rss.get(ResourceID::new(i))); //energy
        res.push_str(sep); //,
    }
    if res.is_empty() {
        for _ in 0..sep.len() {
            res.pop(); //Removes the last separator
        }
    }
    res
}
