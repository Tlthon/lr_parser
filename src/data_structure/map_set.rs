use std::{collections::{HashSet, HashMap}, hash::Hash, ops::Deref};
use std::marker::PhantomData;
use std::ops::BitOr;
use crate::data_structure::joinable::JoinAble;

#[derive(Debug, Clone)]
pub struct MapSet<Key, Value, T: JoinAble = HashSet<Value>> {
    pub(super) content: HashMap<Key, T>,
    default_set: T,
    marker: PhantomData<Value>
}

impl<Key, Value, T: JoinAble> Default for MapSet<Key, Value, T> {
    fn default() -> Self {
        Self { content: HashMap::new() , default_set: T::default(), marker: PhantomData::default()}
    }
}

impl<Key: Clone + Eq + Hash, Value, T: JoinAble<Content = Value>> MapSet<Key, Value, T> {
    pub fn new(key: Key, values: T) -> Self {
        let mut default = Self::default();
        default.content.entry(key).or_default().append(values);
        default
    }
}



impl<Key, Value, T> MapSet<Key, Value, T>
where Key: Hash + Eq,
      T: JoinAble{
    pub fn get(&self, key: &Key) -> &T {
        self.content.get(key).unwrap_or(&self.default_set)
    }
}
impl<Key, Value, T, Content> MapSet<Key, Value, T>
where Key: Hash + Eq 
    , Value: Hash+ Eq
    , T: JoinAble<Content = Content>
{
    pub fn add(&mut self, key: Key, value: Content){
        self.content.entry(key).or_default().insert(value);
    } 
    pub fn append(&mut self, key: Key, values: T){
        self.content.entry(key).or_default().append(values)
    } 
}

impl <'a, Key: Hash + Eq + 'a, Value: Hash + Eq + Clone> MapSet<Key, Value, HashSet<Value>> {
    pub fn aggregate(&self, keys: impl IntoIterator<Item = &'a Key>) -> Option<HashSet<Value>> {
        let values = keys.into_iter().filter_map(|key:&Key| self.content.get(key));
        let value:Option<HashSet<Value>> = values.fold(Some(HashSet::default()), |content, content2| Some(content?.bitor(content2)));
        value
    }
}

impl <Key: Hash + Eq, Value: Hash + Eq + Clone> MapSet<Key, Value, HashSet<Value>> {
    pub fn all(&self) -> Option<HashSet<Value>> {
        let values = self.content.values();
        let value:Option<HashSet<Value>> = values.fold(Some(HashSet::default()), |content, content2| Some(content?.bitor(content2)));
        value
    }
}


impl<Key, Key2, Value> MapSet<Key, Value, MapSet<Key2, Value>>
    where Key: Hash + Eq + Copy,
    Key2: Hash + Eq+ Copy,
    Value: Hash+ Eq + Copy
{
    pub fn add_once(&mut self, key: Key, sub_key: Key2, value: Value){
        self.content.entry(key).or_default().content.entry(sub_key).or_default().insert(value);
    }
}


impl<Key, Value, T> MapSet<Key, Value, T>
    where Key: Hash + Eq
    , Value: Hash+ Eq + Clone
    , T: JoinAble + Clone{
    pub fn join(&mut self, output: Key, source: Key) {
        let Some(first) = self.content.get(&source).cloned() else {
            return;
        };
        self.content.entry(output).or_default().append(first);
    }
}

impl<Key, Value: Hash + Eq> Deref for MapSet<Key, Value, HashSet<Value>> {
    type Target = HashMap<Key, HashSet<Value>>;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
impl<Key: Eq + Hash + Copy, Key2: Eq+Hash+Copy, Value: Hash + Eq + Copy> Deref for MapSet<Key, Value, MapSet<Key2, Value>> {
    type Target = HashMap<Key, MapSet<Key2, Value>>;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}