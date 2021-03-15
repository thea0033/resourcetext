use crate::ui::io::ansi;

use crate::component::ComponentDict;
use crate::object::Object;
use crate::resources::ResourceDict;

impl Object {
    pub fn display_extras(&self, rss: &ResourceDict, cmp: &ComponentDict) -> String {
        let mut res = String::new();
        res.push_str(&self.name);
        res.push('\n');
        res.push_str(&format!("Location: ({}, {})\n", self.location.x, self.location.y));
        res.push_str(&self.resources.display(rss, &self.past));
        res.push_str(&format!("{}Components: \n", ansi::RESET));
        res.push_str(&cmp.display_contained(&self.component_amounts));
        res
    }
} //Displays the object. Pretty self-explanatory.
