#![deny(missing_docs)]
//! A simple in memory key-value store
use std::collections::HashMap;

/// `KvStore` stores string key-value pairs.
///
/// Key-value pairs are stored in memory, and do not persist on disk.
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("foo".to_owned(), "bar".to_owned());
/// let val = store.get("foo".to_owned());
/// assert_eq!(val, Some("bar".to_owned()));
///
/// ```
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Create an empty `KvStore`
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Sets the value of the given key.
    ///
    /// Overwrites the existing value if one exists.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Returns the value of the given key
    ///
    /// Returns `None` if the key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes the given key
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
