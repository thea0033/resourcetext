use std::collections::HashMap;

use crate::merge::Merge;

use super::{ResourceDict, ResourceID, Resources};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReadableResourceDict {
    resources: Vec<ReadableResource>,
    growth: HashMap<String, f64>,
    requirements: HashMap<String, HashMap<String, f64>>,
    transfer_resource: Option<String>,
}
impl ReadableResourceDict {
    pub fn new(
        resources: Vec<ReadableResource>, growth: HashMap<String, f64>, requirements: HashMap<String, HashMap<String, f64>>,
        transfer_resource: Option<String>,
    ) -> Self {
        Self {
            resources,
            growth,
            requirements,
            transfer_resource,
        }
    }
    pub fn default() -> Self {
        Self {
            resources: Vec::new(),
            growth: HashMap::new(),
            requirements: HashMap::new(),
            transfer_resource: None,
        }
    }
    pub fn to_usable(&self) -> Option<ResourceDict> {
        let temp = self.growth.iter();
        let mut growth: HashMap<ResourceID, f64> = HashMap::new();
        for (x, y) in temp {
            let pos = self.resources.iter().position(|s| &s.name == x)?;
            growth.insert(ResourceID::new(pos), *y);
        }
        let mut requirements: HashMap<ResourceID, HashMap<ResourceID, f64>> = HashMap::new();
        let temp = self.requirements.iter();
        for (x, y) in temp {
            let pos = self.resources.iter().position(|s| &s.name == x)?;
            let mut new_y: HashMap<ResourceID, f64> = HashMap::new();
            let temp = y.iter();
            for (x, y) in temp {
                let pos = self.resources.iter().position(|s| &s.name == x)?;
                new_y.insert(ResourceID::new(pos), *y);
            }
            requirements.insert(ResourceID::new(pos), new_y);
        }
        let mut names: Vec<String> = Vec::new();
        let mut transfer_costs: Vec<u64> = Vec::new();
        for line in &self.resources {
            names.push(line.name.clone());
            transfer_costs.push(line.transfer_cost);
        }
        Some(ResourceDict {
            names,
            transfer_costs,
            growth,
            transfer_resource: match &self.transfer_resource {
                Some(val) => {
                    let pos = self.resources.iter().position(|s| &s.name == val)?;
                    Some(ResourceID::new(pos))
                }
                None => None,
            },
            requirements,
        })
    }
}
impl Merge for ReadableResourceDict {
    fn merge(&mut self, other: Self) {
        self.resources.merge(other.resources);
        self.growth.merge(other.growth);
        self.requirements.merge(other.requirements);
        if let Some(val) = other.transfer_resource {
            self.transfer_resource = Some(val);
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ReadableResource {
    name: String,
    transfer_cost: u64,
}

impl ReadableResource {
    pub fn new(name: String, transfer_cost: u64) -> Self {
        Self { name, transfer_cost }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableResources {
    current: HashMap<String, u64>,
    storage: HashMap<String, u64>,
    surplus: HashMap<String, i64>,
}
impl ReadableResources {
    pub fn convert(&self, rss: &ResourceDict) -> Result<Resources, String> {
        let mut res = Resources::new(rss.len());
        for (name, amt) in &self.current {
            let id = rss.find(name).ok_or_else(|| format!("{} is not in the resource dictionary!", name))?;
            res.change_amt(id, *amt);
        }
        for (name, amt) in &self.storage {
            let id = rss.find(name).ok_or_else(|| format!("{} is not in the resource dictionary!", name))?;
            res.change_cap(id, *amt);
        }
        for (name, amt) in &self.surplus {
            let id = rss.find(name).ok_or_else(|| format!("{} is not in the resource dictionary!", name))?;
            res.change_surplus(id, *amt);
        }
        Ok(res)
    }
}
impl Merge for ReadableResources {
    fn merge(&mut self, other: Self) {
        self.current.merge(other.current);
        self.storage.merge(other.storage);
        self.surplus.merge(other.surplus);
    }
}
