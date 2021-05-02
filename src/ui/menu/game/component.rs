use crate::{
    component::ComponentID,
    extra_bits::filter,
    save::Package,
    systems::object_id::ObjectID,
    ui::menu::{config::Config, constants, options::OptionTable},
};

impl Package {
    /// Prompts the user to select components. If max_amounts is set to none, no restrictions are applied. Otherwise, the restrictions are max_amount of each component.
    pub fn select_components(&mut self, config: &mut Config, max_amounts: Option<Vec<usize>>) -> Option<(ComponentID, usize)> {
        let numbered = if let Some(val) = &max_amounts {
            self.cmp.display_contained(val)
        } else {
            self.cmp.display()
        };
        let options: OptionTable = OptionTable::new("Select a component".to_string(), numbered, config.context.grab(constants::SELECT));
        // TODO: Re-implement pasting
        if let Some(val) = max_amounts {
            let id = self.generic_select(config, &options, None, |x| {
                ComponentID::new(filter(x, &val.iter().map(|x| x != &0).collect()))
            })?;
            println!("How many components do you want to select? (between 0 and {})", val[id.id()]);
            let amount: usize = config
                .buffer
                .get_valid_flush(&format!("Please enter a valid number (between 0 and {}). ", val[id.id()]), |x| {
                    *x <= val[id.id()]
                });
            Some((id, amount))
        } else {
            let id = self.generic_select(config, &options, None, ComponentID::new)?;
            println!("How many components do you want to select?");
            let amount = config.buffer.get_flush("Please enter a valid number. ");
            Some((id, amount))
        }
    }
    /// Gets a component from the ui.
    pub fn select_component(&mut self, config: &mut Config, max_amounts: Option<Vec<usize>>) -> Option<ComponentID> {
        let numbered = if let Some(val) = &max_amounts {
            self.cmp.display_contained(val)
        } else {
            self.cmp.display()
        };
        let options: OptionTable = OptionTable::new("Select a component".to_string(), numbered, config.context.grab(constants::SELECT));
        // TODO: Re-implement pasting
        if let Some(val) = max_amounts {
            self.generic_select(config, &options, None, |x| {
                ComponentID::new(filter(x, &val.iter().map(|x| x != &0).collect()))
            })
        } else {
            self.generic_select(config, &options, None, ComponentID::new)
        }
    }
    /// Installs a component from the UI.
    pub fn install_components(&mut self, config: &mut Config, obj: ObjectID) {
        let amts = self.sys.get_object(obj).can_afford(&self.cmp);
        if let Some((id, amt)) = self.select_components(config, Some(amts)) {
            let installed = self.sys.get_object_mut(obj).install_components(id, &self.cmp, amt);
            if installed == amt {
                self.message(config, &format!("{} components successfully installed!", amt));
            } else {
                self.message(config, &format!("Could only install {} out of {} components!", installed, amt));
            }
        }
    }
    pub fn remove_components(&mut self, config: &mut Config, obj: ObjectID) {
        let amts = self.sys.get_object(obj).get_cmp_amts().clone();
        if let Some((id, amt)) = self.select_components(config, Some(amts)) {
            let installed = self.sys.get_object_mut(obj).remove_components(id, &self.cmp, amt);
            if installed == amt {
                self.message(config, &format!("{} components successfully removed!", amt));
            } else {
                self.message(config, &format!("Could only remove {} out of {} components!", installed, amt));
            }
        } else {
            self.aborted(config);
        }
    }
    pub fn aborted(&self, config: &mut Config) {
        println!("Operation aborted successfully");
        config.buffer.read();
    }
    pub fn message(&self, config: &mut Config, msg: &str) {
        println!("{}", msg);
        config.buffer.read();
    }
}
