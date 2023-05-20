use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
/// A simple wrap around an HashMap.
pub struct Table<K: Eq + Hash, V> {
    core: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Default for Table<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> Table<K, V> {
    /// Create a new table.
    pub fn new() -> Table<K, V> {
        Table {
            core: HashMap::new(),
        }
    }

    /// Insert a value of type V with key of type K in the table.
    /// Return false if a value for the key is already in the table,
    /// true otherwise.
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.core.entry(key) {
            e.insert(value);
            true
        } else {
            false
        }
    }

    /// Return a value if a key was found in the table.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.core.get(key)
    }

    /// Return true if the table has the key.
    pub fn has(&self, key: &K) -> bool {
        self.core.contains_key(key)
    }
}

#[derive(Clone)]
/// A scoped wrap around an HashMap.
pub struct ScopedTable<K: Eq + Hash, V> {
    scopes: Vec<Table<K, V>>,
}

impl<K: Eq + Hash, V> Default for ScopedTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> ScopedTable<K, V> {
    /// Create a new empty ScopedTable.
    pub fn new() -> ScopedTable<K, V> {
        ScopedTable { scopes: vec![] }
    }
    /// Insert a value V with key K in the most recent scope.
    /// Return Err if no scope is open, Ok(false) if the scope
    /// is open but it was not possible to insert a new value for T
    /// (i.e. a key was already present) and Ok(true) if
    /// the scope is open and the value was inserted.
    pub fn insert(&mut self, key: K, value: V) -> anyhow::Result<bool> {
        if self.scopes.is_empty() {
            Err(anyhow::anyhow!("Cannot insert: no scope open"))
        } else if self.scopes.last().unwrap().has(&key) {
            Ok(false)
        } else {
            self.scopes.last_mut().unwrap().insert(key, value);
            Ok(true)
        }
    }

    /// Get a value V with key K in the most recent scope.
    /// Return Err if no scope is open, Ok(None)  if the scope
    /// is open but no value for key was found and Ok(Some(&value)) if
    /// the scope is open and the value was inserted.
    pub fn get(&self, key: K) -> anyhow::Result<Option<&V>> {
        if self.scopes.is_empty() {
            Err(anyhow::anyhow!("Cannot get: no scope open"))
        } else {
            for scope in &self.scopes {
                if scope.has(&key) {
                    return Ok(scope.get(&key));
                }
            }

            Ok(None)
        }
    }

    /// Check if the scoped table has a key.
    /// Return Err if no scope is open, Ok(false) if no
    /// value for key is found in any scope and Ok(true) otherwise.
    pub fn has(&self, key: K) -> anyhow::Result<bool> {
        if self.scopes.is_empty() {
            Err(anyhow::anyhow!("Cannot get: no scope open"))
        } else {
            for scope in &self.scopes {
                if scope.has(&key) {
                    return Ok(true);
                }
            }

            Ok(false)
        }
    }

    /// Open a new scope.
    pub fn open(&mut self) {
        self.scopes.push(Table::new());
    }

    /// Close the most recent scope.
    pub fn close(&mut self) {
        self.scopes.pop();
    }

    /// Check if the most recent scope has a key.
    /// Return Err if no scope is open, Ok(false) if no
    /// value for key is found and Ok(true) otherwise.
    pub fn has_here(&self, key: K) -> anyhow::Result<bool> {
        if self.scopes.is_empty() {
            Err(anyhow::anyhow!("Cannot get: no scope open"))
        } else {
            Ok(self.scopes.last().unwrap().has(&key))
        }
    }
}
