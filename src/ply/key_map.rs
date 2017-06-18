use linked_hash_map::LinkedHashMap;
use super::ElementDef;
use super::PropertyDef;

pub type KeyMap<V> = LinkedHashMap<String, V>;

pub trait Addable<V: Key> {
    fn add(&mut self, new_value: V);
}


impl<V: Key> Addable<V> for KeyMap<V> {
    fn add(&mut self, value: V) {
        self.insert(value.get_key(), value);
    }
}

pub trait Key {
    fn get_key(&self) -> String;
}
impl Key for ElementDef {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

impl Key for PropertyDef {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}
/*
pub trait Access<V> {
    fn last(&self) -> Option<&V>;
}

impl<V> Access<V> for KeyMap<V> {
    fn last(&self) -> Option<&V> {
        match self.iter().last() {
            None => None,
            Some((_, v)) => Some(v),
        }
    }
}
*/
