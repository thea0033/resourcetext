use crate::{save::Package, systems::object_id::ObjectID, ui::menu::{config::Config, constants, options::OptionTable}};
use crate::component::RecipeID;
use crate::extra_bits::filter;

impl Package {
    pub fn select_recipes(&mut self, config: &mut Config, max_amounts: Option<Vec<usize>>) -> Option<(RecipeID, usize)> {
        let numbered = if let Some(val) = &max_amounts {
            self.cmp.display_contained_r(val)
        } else {
            self.cmp.display_r()
        };
        let options: OptionTable = OptionTable::new("Select a recipe".to_string(), numbered, config.context.grab(constants::SELECT));
        // TODO: Re-implement pasting
        if let Some(val) = max_amounts {
            let id = self.generic_select(config, &options, None, |x| {
                RecipeID::new(filter(x, &val.iter().map(|x| x != &0).collect()))
            })?;
            println!("How many recipes do you want to select? (between 0 and {})", val[id.id()]);
            let amount: usize = config
                .buffer
                .get_valid_flush(&format!("Please enter a valid number (between 0 and {}). ", val[id.id()]), |x| {
                    *x <= val[id.id()]
                });
            Some((id, amount))
        } else {
            let id = self.generic_select(config, &options, None, RecipeID::new)?;
            println!("How many recipes do you want to select?");
            let amount = config.buffer.get_flush("Please enter a valid number. ");
            Some((id, amount))
        }
        
    }
    pub fn perform_recipe(&mut self, config: &mut Config, obj: ObjectID) {
        let amts = self.sys.get_object(obj).can_afford_recipes(&self.cmp);
        if let Some((id, amt)) = self.select_recipes(config, Some(amts)) {
            let installed = self.sys.get_object_mut(obj).do_recipes(id, &self.cmp, amt);
            if installed == amt {
                self.message(config, &format!("{} recipes successfully performed!", amt));
            } else {
                self.message(config, &format!("Could only perform {} out of {} recipes!", installed, amt));
            }
        }
    }
}
