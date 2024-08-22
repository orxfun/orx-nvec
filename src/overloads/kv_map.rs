use alloc::collections::btree_map::BTreeMap;
use core::hash::Hash;
use std::collections::HashMap;

pub trait KvMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K: Ord, V> KvMap<K, V> for BTreeMap<K, V> {
    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

#[cfg(feature = "std")]
impl<K: Hash + Eq, V> KvMap<K, V> for HashMap<K, V> {
    #[inline]
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}
