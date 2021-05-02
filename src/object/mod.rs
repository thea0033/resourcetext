use crate::component::*;
use crate::resources::*;
use crate::{location::*, systems::system_id::*};

use self::template::Template;
mod component;
mod display;
pub mod readable;
pub mod template;
mod tick;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Object {
    location: Location,              //The object's current location.
    resources: Resources,            //The resources the object has.
    past: Resources,                 //The resources the object had last tick.
    component_amounts: Vec<usize>,   //Tracker for each component.
    h_component_amounts: Vec<usize>, //Tracker for each hidden component.
    name: String,                    //The object's name.
    system: SystemID,                //What system the object's in.
} //The structure for an object. Objects are ships, planets, even projectiles.
impl Object {
    /// Creates a new object with no components.
    pub fn new(rss: &ResourceDict, cmp: &ComponentDict, name: String, loc: Location, sys: SystemID) -> Object {
        Object {
            location: loc,
            resources: Resources::new(rss.len()),
            past: Resources::new(rss.len()),
            component_amounts: vec![0; cmp.list.len()],
            h_component_amounts: vec![0; cmp.hidden_list.len()],
            name,
            system: sys,
        }
    }
    /// Gets the location of the object mutably
    pub fn get_location_mut(&mut self) -> &mut Location {
        &mut self.location
    }
    /// Gets the location of the object
    pub fn get_location(&self) -> &Location {
        &self.location
    }
    /// Gets the amounts of components currently stored.
    pub fn get_cmp_amts(&self) -> &Vec<usize> {
        &self.component_amounts
    }
    /// Gets a reference to the resources in an object.
    pub fn resources(&self) -> &Resources {
        &self.resources
    }
    /// Gets a mutable reference to the resources in an object.
    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
    /// Converts the object to a template.
    pub fn to_template(&self, cmp: &ComponentDict, rss: &ResourceDict, name: String) -> Template {
        let mut surplus: Vec<i64> = vec![0; rss.len()];
        let mut storage: Vec<u64> = vec![0; rss.len()];
        let mut cost: Vec<i64> = vec![0; rss.len()]; //initializes vectors
        let mut transfer_cost: u64 = 0;
        for (i, line) in self.component_amounts.iter().enumerate() {
            let c = cmp.get(ComponentID::new(i));
            for (i, s) in c.surplus().iter().enumerate() {
                surplus[i] += s * (*line as i64);
            }
            for (i, s) in c.storage().iter().enumerate() {
                storage[i] += s * (*line as u64);
            }
            for (i, s) in c.cost().iter().enumerate() {
                cost[i] += s * (*line as i64);
            }
        } //Calculates cost and surplus
        let mut flag = true;
        for (i, item) in cost.iter().enumerate() {
            if *item > 0 {
                if rss.get_transfer_costs()[i] == u64::MAX {
                    //If we aren't supposed to be able to transfer something...
                    flag = false;
                    break;
                }
                transfer_cost += (*item as u64) * rss.get_transfer_costs()[i] as u64;
                //NOTE: The casting is SAFE
            }
        } //Calculates transfer cost based on all negative costs.
        Template::new(
            self.component_amounts.clone(),
            name,
            surplus,
            storage,
            cost,
            if flag { Some(transfer_cost) } else { None },
        )
    }
    /// Gets the amount of each component you can afford.
    pub fn can_afford(&self, cmp: &ComponentDict) -> Vec<usize> {
        cmp.list.iter().map(|x| self.resources().amt_contained(x.cost())).collect()
    }
    pub fn can_afford_recipes(&self, cmp: &ComponentDict) -> Vec<usize> {
        cmp.recipe_list.iter().map(|x| self.resources().amt_contained(x.cost())).collect()
    }
}
