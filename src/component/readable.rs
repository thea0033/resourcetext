use std::collections::HashMap;

use crate::resources::{ResourceDict, ResourceID};

use super::{recipe::Recipe, Component, Components};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableComponents {
    accessible: HashMap<String, ReadableComponent>,
    hidden: HashMap<String, ReadableComponent>,
    recipes: HashMap<String, ReadableRecipe>,
}
impl ReadableComponents {
    pub fn convert(self, rss: &ResourceDict) -> Option<Components> {
        let mut cmp: Components = Components::new();
        let mut names: Vec<String> = Vec::new();
        let mut buffer: Vec<Component> = Vec::new();
        for (i, line) in self.accessible {
            names.push(i);
            buffer.push(line.convert(rss)?);
        }
        cmp.add_l(names, buffer);
        let mut names: Vec<String> = Vec::new();
        let mut buffer: Vec<Component> = Vec::new();
        for (i, line) in self.hidden {
            names.push(i);
            buffer.push(line.convert(rss)?);
        }
        cmp.add_h_l(names, buffer);
        let mut names: Vec<String> = Vec::new();
        let mut buffer: Vec<Recipe> = Vec::new();
        for (i, line) in self.recipes {
            names.push(i);
            buffer.push(line.convert(rss)?);
        }
        cmp.add_r_l(names, buffer);
        Some(cmp)
    }
}
impl Components {
    pub fn into_readable(self, rss: &ResourceDict) -> ReadableComponents {
        let mut accessible: HashMap<String, ReadableComponent> = HashMap::new();
        let mut hidden: HashMap<String, ReadableComponent> = HashMap::new();
        let mut recipes: HashMap<String, ReadableRecipe> = HashMap::new();
        for (line, name) in self.list.into_iter().zip(self.names.into_iter()) {
            accessible.insert(name, line.to_readable(rss));
        }
        for (line, name) in self.hidden_list.into_iter().zip(self.hidden_names.into_iter()) {
            hidden.insert(name, line.to_readable(rss));
        }
        for (line, name) in self.recipe_list.into_iter().zip(self.recipe_names.into_iter()) {
            recipes.insert(name, line.to_readable(rss));
        }
        ReadableComponents { accessible, hidden, recipes }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableComponent {
    surplus: HashMap<String, i64>,
    storage: HashMap<String, u64>,
    cost: HashMap<String, i64>,
}
impl ReadableComponent {
    pub fn convert(&self, rss: &ResourceDict) -> Option<Component> {
        let mut res: Component = Component::new(rss.len());
        for (id, line) in &self.cost {
            res.change_cost(rss.find(id)?, *line);
        }
        for (id, line) in &self.surplus {
            res.change_surplus(rss.find(id)?, *line);
        }
        for (id, line) in &self.storage {
            res.change_storage(rss.find(id)?, *line);
        }
        Some(res)
    }
}
impl Component {
    pub fn to_readable(&self, rss: &ResourceDict) -> ReadableComponent {
        let mut surplus: HashMap<String, i64> = HashMap::new();
        let mut storage: HashMap<String, u64> = HashMap::new();
        let mut cost: HashMap<String, i64> = HashMap::new();
        for (i, line) in self.surplus.iter().enumerate() {
            if *line != 0 {
                surplus.insert(rss.get(ResourceID::new(i)), *line);
            }
        }
        for (i, line) in self.storage.iter().enumerate() {
            if *line != 0 {
                storage.insert(rss.get(ResourceID::new(i)), *line);
            }
        }
        for (i, line) in self.cost.iter().enumerate() {
            if *line != 0 {
                cost.insert(rss.get(ResourceID::new(i)), *line);
            }
        }
        ReadableComponent { surplus, storage, cost }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableRecipe {
    cost: HashMap<String, i64>,
}
impl ReadableRecipe {
    pub fn convert(&self, rss: &ResourceDict) -> Option<Recipe> {
        let mut res: Recipe = Recipe::new(rss.len());
        for (id, line) in &self.cost {
            res.cost()[rss.find(&id)?.get()] = *line;
        }
        Some(res)
    }
}
impl Recipe {
    pub fn to_readable(&self, rss: &ResourceDict) -> ReadableRecipe {
        let mut cost: HashMap<String, i64> = HashMap::new();
        for (i, line) in self.cost.iter().enumerate() {
            if *line != 0 {
                cost.insert(rss.get(ResourceID::new(i)), *line);
            }
        }
        ReadableRecipe { cost }
    }
}
