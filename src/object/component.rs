use crate::component::ComponentDict;
use crate::component::ComponentID;
use crate::{component::RecipeID, object::Object, resources::ResourceID, ui::io::ansi};

impl Object {
    pub fn install_components(&mut self, id: ComponentID, cmp: &ComponentDict, amt: usize) -> usize {
        let component = cmp.get(id); //Gets component
        if !self.resources.spend(&component.cost().iter().map(|x| x * (amt as i64)).collect()) {
            //Attempts to spend all resources at once. If this fails...
            for i in 0..amt {
                //Attemts to install the components one at a time.
                if !self.resources.spend(component.cost()) {
                    //Attempts to spend resources required for one component. If that fails...
                    return i; //We could only do so many.
                }
                if id.is_hidden() {
                    self.h_component_amounts[id.id()] += 1;
                } else {
                    self.component_amounts[id.id()] += 1;
                } //Registers an increase on either the list or the hidden list.
                self.resources.add_storage_vec(component.storage());
                self.resources.add_surplus_vec(component.surplus()); //Finalizes the component's
                                                                     // installation by adding its
                                                                     // surplus and storage bonuses
            }
        } else {
            //If the spending succeeds, finalize it.
            self.resources
                .add_storage_vec(&component.storage().iter().map(|x| x * (amt as u64)).collect());
            self.resources
                .add_surplus_vec(&component.surplus().iter().map(|x| x * (amt as i64)).collect());
            if id.is_hidden() {
                self.h_component_amounts[id.id()] += 1;
            } else {
                self.component_amounts[id.id()] += 1;
            }
        }
        amt //We did all of the installations!
    }
    pub fn do_recipes(&mut self, id: RecipeID, cmp: &ComponentDict, amt: usize) -> usize {
        let recipe = cmp.get_r(id);
        if !self.resources.spend(&recipe.cost_stat().iter().map(|x| x * (amt as i64)).collect()) {
            //Attempts to perform all of the recipes at once. If that fails...
            for i in 0..amt {
                //Attempts to do them one at a time!
                if !self.resources.spend(recipe.cost_stat()) {
                    //Attempts to perform the recipe once, in a loop
                    return i; //If it fails, return the amount of successes
                              // before that.
                }
            }
        }
        amt
    }
    pub fn force_install_components(&mut self, id: ComponentID, cmp: &ComponentDict, amt: u64) {
        self.past = self.resources.clone(); //"backs up" the current resource amount
        let component = cmp.get(id); //Gets component
        self.resources.force_spend(&component.cost().iter().map(|x| x * (amt as i64)).collect()); //Forcefully spends all required resources at once
        self.resources
            .add_storage_vec(&component.storage().iter().map(|x| x * (amt as u64)).collect()); //Adds the storage benefits of the component.
        self.resources
            .add_surplus_vec(&component.surplus().iter().map(|x| x * (amt as i64)).collect()); //Adds the surplus benefits of the component.
        if id.is_hidden() {
            self.h_component_amounts[id.id()] += 1;
        } else {
            self.component_amounts[id.id()] += 1;
        }
    }
    pub fn remove_components(&mut self, id: ComponentID, cmp: &ComponentDict, amt: usize) -> usize {
        for i in 0..amt {
            let component = cmp.get(id);
            if !self.resources.gain(component.cost())
                || self.component_amounts[id.id()] == 0
                || !self.resources.can_rmv_storage_vec(component.storage())
            {
                //If we can't remove a component (e.g it provides benefits we can't do w/o, or
                // we don't have any to remove)
                return i; //Can't re-spend gains
            } //Spends the required stuff; if it can't, returns the amount already installed.
              //Otherwise...
            if id.is_hidden() {
                self.h_component_amounts[id.id()] -= 1;
            } else {
                self.component_amounts[id.id()] -= 1;
            }
            self.resources.rmv_storage_vec(component.storage());
            self.resources.rmv_surplus_vec(component.surplus());
        }

        amt
    }
    pub fn name(&self) -> &str {
        &self.name
    } //Returns the object's name
    pub fn color(&self) -> &str {
        let curr = &self.resources;
        let past = &self.past;
        let mut all_zero: bool = true;
        for line in 0..curr.get_currs().len() {
            let c = curr.get_curr(ResourceID::new(line)); //Current
            let p = past.get_curr(ResourceID::new(line)); //Past
            if c < p {
                //If we're losing resources, it's red
                return ansi::RED;
            }
            if c > p {
                //If have a surplus, it isn't zero
                all_zero = false;
            }
        }
        if all_zero {
            return ansi::YELLOW; //If every resource is flat, our color is
                                 // yellow
        }
        ansi::GREEN //Otherwise, it's green
    } //Returns the color of the object.
}
