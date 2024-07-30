use std::collections::HashMap;
use std::hash::Hash;

pub trait MultiMap<K: Hash + Eq, V> {
    fn from_multi_iter<I: IntoIterator<Item = (K, V)>>(iterator: I) -> Self;
}

impl<K: Hash + Eq, V> MultiMap<K, V> for HashMap<K, Vec<V>> {
    fn from_multi_iter<I: IntoIterator<Item = (K, V)>>(iterator: I) -> Self {
        let mut map = Self::new();
        for (key, value) in iterator {
            match map.get_mut(&key) {
                None => {
                    map.insert(key, vec![value]);
                }
                Some(values) => {
                    values.push(value);
                }
            }
        }
        map
    }
}
