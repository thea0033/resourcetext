use crate::{
    extra_bits,
    resources::{ResourceDict, ResourceID},
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub cost: Vec<i64>,
}
impl Recipe {
    pub fn new(len: usize) -> Recipe {
        Recipe {
            cost: vec![0; len],
        }
    }
    pub fn cost(&mut self) -> &mut Vec<i64> {
        &mut self.cost
    }
    pub fn cost_stat(&self) -> &Vec<i64> {
        &self.cost
    }
    pub fn display(&self, rss: &ResourceDict) -> String {
        let mut positives = Vec::new();
        let mut negatives = Vec::new();
        for (i, item) in self.cost.iter().enumerate() {
            match item.cmp(&0) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    positives.push(i);
                }
                std::cmp::Ordering::Less => {
                    negatives.push(i);
                }
            }
        }
        let mut res: String = "".to_string();
        let p_len = positives.len();
        let n_len = negatives.len();
        if p_len != 0 {
            res.push_str("Costs: ");
            for line in positives {
                res.push_str(&self.cost[line].to_string());
                res.push(' ');
                res.push_str(&rss.get(ResourceID::new(line)));
                res.push(',');
            }
            res.pop();
            res.push(' ');
            res.push('\n');
        }
        if n_len != 0 {
            res.push_str("Gains: ");
            for line in negatives {
                res.push_str(&(-self.cost[line]).to_string());
                res.push(' ');
                res.push_str(&rss.get(ResourceID::new(line)));
                res.push(',');
            }
            res.pop();
            res.push(' ');
        }
        if p_len == 0 && n_len == 0 {
            res.push_str("Empty recipe");
        }
        res
    }
}
