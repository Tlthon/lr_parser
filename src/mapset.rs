use std::{collections::{HashSet, HashMap}, hash::Hash, ops::Deref};

#[derive(Debug)]
pub struct MapSet<Key, Value>{
    content: HashMap<Key, HashSet<Value>>,
    default_set: HashSet<Value>,
}

impl<Key, Value> Default for MapSet<Key, Value> {
    fn default() -> Self {
        Self { content: HashMap::new() , default_set: HashSet::new()}
    }
}

impl<Key, Value> MapSet<Key, Value> {

    pub fn add(&mut self, key: Key, value: Value) -> ()
    where   Key: Hash + Eq,
            Value: Hash+ Eq
    {
        self.content.entry(key).or_default().insert(value);
    } 
    pub fn set(&mut self, key: Key, values: HashSet<Value>)
    where   Key: Hash + Eq,
            Value: Hash+ Eq
    {
        self.content.insert(key, values);
    } 

    pub fn join(&mut self, output: Key, source: Key) 
    where   Key: Hash + Eq, HashSet<Value>: Clone, Value: Eq + Hash+ Clone{
        let Some(first) = self.content.get(&source).cloned()else {
            return;
        };
        self.content.entry(output).or_default().extend(first);
    }

    pub fn append(&mut self, key: Key, set: HashSet<Value>)
    where   Key: Hash + Eq, Value: Eq + Hash
    {
        self.content.entry(key).or_default().extend(set)
    } 

    pub fn get(&self, key: &Key) -> &HashSet<Value>
    where Key: Eq + Hash {
        self.content.get(key).unwrap_or(&self.default_set)
    }
}

impl<Key, Value> Deref for MapSet<Key, Value> {
    type Target = HashMap<Key, HashSet<Value>>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}