use std::collections::HashSet;
use std::hash::Hash;
use crate::data_structure::map_set::MapSet;

pub trait JoinAble: Default {
    type Content;
    fn append(&mut self, other: Self );
    fn insert(&mut self, new_value: Self::Content);
}

impl<T: Hash + Eq> JoinAble for HashSet<T>{
    type Content = T;
    fn append(&mut self, other: Self) {
        // self.extend(other.into_iter());

        Extend::extend(self, other);
    }
    fn insert(&mut self, new_value: Self::Content) {
        self.insert(new_value);
    }
}

impl <K: Eq + Hash + Copy,V: Eq + Hash + Copy> JoinAble for MapSet<K,V>{
    type Content = (K, HashSet<V>);
    fn append(&mut self, other: Self) {
        for (key, value) in other.content.iter() {
            self.content.entry(key.clone()).or_default().append(value.clone());
        }
    }
    fn insert(&mut self, (new_key, new_value): Self::Content) {
        self.content.entry(new_key).or_default().append(new_value);
    }
}