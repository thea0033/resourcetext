use std::{collections::HashMap, ops::AddAssign};
use std::hash::Hash;

pub trait Merge {
    fn merge(&mut self, other: Self);
}
impl<T> Merge for Vec<T> {
    fn merge(&mut self, mut other: Self) {
        self.append(&mut other);
    }
}
impl<T, U> Merge for HashMap<T, U> where U: Merge, T: Eq, T: Hash{
    fn merge(&mut self, other: Self) {
        for (key, line) in other {
            if self.get(&key).is_some() {
                self.get_mut(&key).expect("safe unwrap").merge(line);
            } else {
                self.insert(key, line);
            }
        }
    }
}
impl Merge for u64{
    fn merge(&mut self, other: Self) {
        *self += other;
    }
}
impl Merge for i64{
    fn merge(&mut self, other: Self) {
        *self += other;
    }
}