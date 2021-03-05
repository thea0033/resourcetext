use super::*;
impl ResourceDict {
    pub fn new(
        vals: Vec<String>, t_costs: Vec<u64>, growth: HashMap<ResourceID, f64>, requirements: HashMap<ResourceID, HashMap<ResourceID, f64>>,
        transfer: Option<ResourceID>,
    ) -> ResourceDict {
        ResourceDict {
            names: vals,
            transfer_costs: t_costs,
            growth,
            requirements,
            transfer_resource: transfer,
        }
    } //Basic new function
    pub fn display_filtered_addon<T>(&self, filter: &Vec<bool>, extra_text: &Vec<T>) -> String
    where
        T: Display, {
        let mut res = "".to_string();
        let mut i = 0;
        for j in 0..self.names.len() {
            if filter[j] {
                res.push_str(&format!("{}: {} ({})\n", i, self.names[j], extra_text[j]));
                i += 1;
            }
        }
        res
    } //An add-on to the display function that helps with filtration
    pub fn len(&self) -> usize {
        self.names.len()
    } //Returns the amount of resources
    pub fn get(&self, id: ResourceID) -> String {
        self.names[id.get()].clone()
    } //Returns the resource name
    pub fn get_transfer_costs(&self) -> &Vec<u64> {
        &self.transfer_costs
    } //Returns all of the transfer costs
    pub fn find(&self, name: &str) -> Option<ResourceID> {
        Some(ResourceID::new(self.names.iter().position(|x| x == name)?))
    }
    pub fn get_growth(&self) -> &HashMap<ResourceID, f64> {
        &self.growth
    }
    pub fn get_requirements(&self) -> &HashMap<ResourceID, HashMap<ResourceID, f64>> {
        &self.requirements
    }
    pub fn get_transfer(&self) -> Option<ResourceID> {
        self.transfer_resource
    }
}
