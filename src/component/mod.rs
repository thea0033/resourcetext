pub mod recipe;
pub mod readable;
use std::collections::HashMap;

use crate::extra_bits;
use crate::resources::*;

use self::recipe::Recipe;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Components {
    pub list: Vec<Component>, //list of all accessible components
    pub names: Vec<String>,   //names of all accessible components
    pub hidden_list: Vec<Component>, /* list of all hidden components (hidden = can't install
                               * yourself) */
    pub hidden_names: Vec<String>, //names of all hidden components
    pub recipe_list: Vec<Recipe>,  //list of all recipes
    pub recipe_names: Vec<String>, //names of all recipes
}
impl Components {
    pub fn get(&self, id: ComponentID) -> &Component {
        if !id.is_hidden {
            &self.list[id.id]
        } else {
            &self.hidden_list[id.id]
        }
    } //gets a component from the lists
    pub fn get_from_name(&self, name: &str) -> ComponentID {
        for (i, line) in self.names.iter().enumerate() {
            if line == name {
                return ComponentID::new(i);
            }
        }
        panic!("{} was not found!", name);
    }
    pub fn get_from_name_h(&self, name: &str) -> ComponentID {
        for (i, line) in self.hidden_names.iter().enumerate() {
            if line == name {
                return ComponentID::new_h(i);
            }
        }
        panic!("{} was not found!", name);
    }
    pub fn get_r(&self, id: RecipeID) -> &Recipe {
        &self.recipe_list[id.id]
    } //gets a recipe from the list
    pub fn get_name(&self, id: ComponentID) -> &String {
        if !id.is_hidden {
            &self.names[id.id]
        } else {
            &self.hidden_names[id.id]
        }
    } //gets the component name from the lists
    pub fn get_r_name(&self, id: RecipeID) -> &String {
        &self.recipe_names[id.id]
    } //gets the recipe name from the list
    pub fn new() -> Components {
        Components {
            list: Vec::new(),
            names: Vec::new(),
            hidden_list: Vec::new(),
            hidden_names: Vec::new(),
            recipe_list: Vec::new(),
            recipe_names: Vec::new(),
        }
    } //new function
    pub fn add_l(&mut self, mut name: Vec<String>, mut component: Vec<Component>) {
        self.list.append(&mut component);
        self.names.append(&mut name);
    } //adds a list of components and names to the component dictionary
    pub fn add_h_l(&mut self, mut name: Vec<String>, mut component: Vec<Component>) {
        self.hidden_list.append(&mut component);
        self.hidden_names.append(&mut name);
    } //add_l but in the hidden category
    pub fn add_r_l(&mut self, mut name: Vec<String>, mut recipe: Vec<Recipe>) {
        self.recipe_list.append(&mut recipe);
        self.recipe_names.append(&mut name);
    } //adds a list of recipes and names to the component dictionary
    pub fn display(&self) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.list.len() {
            x.push_str(&format!("{}: {}", i, &self.names[i]));
            x.push('\n'); //separates them by line
        }
        x
    } //displays the accessible components
    pub fn display_contained(&self, a: &Vec<usize>) -> String {
        let mut x: String = "".to_string();
        let mut counter: usize = 0;
        for (i, item) in a.iter().enumerate() {
            if *item != 0 {
                x.push_str(&format!("{}: {} ({})", counter, &self.names[i], item));
                x.push_str(", \n");
                counter += 1;
            }
        }
        x
    } //displays the accessible components, but filters them based on how many of
      // them there are
    pub fn display_detailed(&self, rss: &ResourceDict) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.list.len() {
            x.push_str(&format!("{}: {}", i, &self.names[i]));
            x.push('\n');
            x.push_str(&self.list[i].display(rss));
        }
        x
    }
    pub fn display_one(&self, rss: &ResourceDict, id: ComponentID) -> String {
        let mut x: String = "".to_string();
        x.push_str(&format!("{}: {}", id.id(), &self.names[id.id()]));
        x.push('\n');
        x.push_str(&self.list[id.id()].display(rss));
        x
    }
    pub fn display_r(&self) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.recipe_list.len() {
            x.push_str(&format!("{}: {}", i, &self.recipe_names[i]));
            x.push('\n');
        }
        x
    }
    pub fn display_contained_r(&self, a: &Vec<usize>) -> String {
        let mut x: String = "".to_string();
        let mut counter: usize = 0;
        for (i, item) in a.iter().enumerate() {
            if *item != 0 {
                x.push_str(&format!("{}: {} ({})", counter, &self.recipe_names[i], item));
                x.push_str(", \n");
                counter += 1;
            }
        }
        x
    }
    pub fn display_detailed_r(&self, rss: &ResourceDict) -> String {
        let mut x: String = "".to_string();
        for i in 0..self.recipe_list.len() {
            x.push_str(&format!("{}: {}", i, &self.recipe_names[i]));
            x.push('\n');
            x.push_str(&self.recipe_list[i].display(rss));
            x.push('\n');
        }
        x
    }
    pub fn display_one_r(&self, rss: &ResourceDict, i: RecipeID) -> String {
        let mut x: String = "".to_string();
        x.push_str(&format!("{}: {}", i.id, &self.recipe_names[i.id]));
        x.push('\n');
        x.push_str(&self.recipe_list[i.id].display(rss));
        x
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn len_r(&self) -> usize {
        self.recipe_list.len()
    }
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Component {
    surplus: Vec<i64>,
    storage: Vec<u64>,
    cost: Vec<i64>,
}
impl Component {
    pub fn cost(&self) -> &Vec<i64> {
        &self.cost
    }
    pub fn surplus(&self) -> &Vec<i64> {
        &self.surplus
    }
    pub fn storage(&self) -> &Vec<u64> {
        &self.storage
    }
    pub fn change_cost(&mut self, id: ResourceID, val: i64) {
        self.cost[id.get()] = val;
    }
    pub fn change_surplus(&mut self, id: ResourceID, val: i64) {
        self.surplus[id.get()] = val;
    }
    pub fn change_storage(&mut self, id: ResourceID, val: u64) {
        self.storage[id.get()] = val;
    }
    pub fn new(size: usize) -> Component {
        Component {
            surplus: vec![0; size],
            storage: vec![0; size],
            cost: vec![0; size],
        }
    } //Basic accessing functions
    pub fn display(&self, rss: &ResourceDict) -> String {
        let mut x: String = "".to_string();
        x.push_str(&self.display_func(rss, "  cost: ", &self.cost, 0));
        x.push_str(&self.display_func(rss, "  surplus: ", &self.surplus, 0));
        x.push_str(&self.display_func(rss, "  storage: ", &self.storage, 0));
        x
    }
    pub fn display_func<T>(&self, rss: &ResourceDict, msg: &str, a: &Vec<T>, zero: T) -> String
    where
        T: PartialEq,
        T: Copy,
        T: ToString, {
        let mut x: String = "".to_string(); //Initializes rseult
        let mut flag: bool = false;
        for (i, item) in a.iter().enumerate() {
            //For every resource...
            if *item != zero {
                //Doesn't display resources that you have zero of
                if !flag {
                    //Does this once, if a resource is present
                    x.push_str(msg); //Adds the message parameter
                    flag = true; //Sets flag to true
                }
                x.push_str(&(*item).to_string()); //Adds the amount of the resource
                x.push(' ');
                x.push_str(&rss.get(ResourceID::new(i))); //Adds the resource type
                x.push(',');
                x.push(' '); //Extra formatting stuff
            }
        }
        x.push('\n'); //adds newline character so that everything appears on a separate line
        x //returns result
    } //Displays lots of stuff
}
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ComponentID {
    id: usize,
    is_hidden: bool,
} //Identifies component
impl ComponentID {
    pub fn id(&self) -> usize {
        self.id
    } //getter
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    } //getter
    pub fn new(id: usize) -> ComponentID {
        ComponentID { id, is_hidden: false }
    } //new, hidden set to false
    pub fn new_h(id: usize) -> ComponentID {
        ComponentID { id, is_hidden: true }
    } //new, hidden set to true
}
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RecipeID {
    id: usize,
} //Recipe id wrapper
impl RecipeID {
    pub fn new(id: usize) -> RecipeID {
        RecipeID { id }
    } //new
    pub fn id(&self) -> usize {
        self.id
    }
}
