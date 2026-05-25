use std::collections::{HashMap, HashSet};

pub struct NeoCollection {
    pub name: String,
    pub records: HashMap<String, String>,
    pub indexes: HashMap<String, HashMap<String, HashSet<String>>>,
    pub reverse_indexes: HashMap<String, HashMap<String, String>>,
}

impl NeoCollection {
    pub fn new(name: String) -> Self {
        Self {
            name,
            records: HashMap::new(),
            indexes: HashMap::new(),
            reverse_indexes: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.records.get(key)
    }

    pub fn put(&mut self, key: String, value: String, indexes: Option<HashMap<String, String>>) {
        self.delete_indexes(&key);
        if let Some(idx_map) = indexes {
            let mut rev_idx = HashMap::with_capacity(idx_map.len());
            for (index_name, index_value) in idx_map {
                self.indexes
                    .entry(index_name.clone())
                    .or_default()
                    .entry(index_value.clone())
                    .or_default()
                    .insert(key.clone());

                rev_idx.insert(index_name, index_value);
            }
            self.reverse_indexes.insert(key.clone(), rev_idx);
        }
        self.records.insert(key, value);
    }

    fn delete_indexes(&mut self, key: &str) {
        if let Some(index_to_remove) = self.reverse_indexes.remove(key) {
            for (index_name, index_value) in index_to_remove {
                if let Some(inner_map) = self.indexes.get_mut(&index_name) {
                    if let Some(keys_set) = inner_map.get_mut(&index_value) {
                        keys_set.remove(key);
                        if keys_set.is_empty() {
                            inner_map.remove(&index_value);
                        }
                    }
                    if inner_map.is_empty() {
                        self.indexes.remove(&index_name);
                    }
                }
            }
        }
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.delete_indexes(key);
        self.records.remove(key).is_some()
    }

    pub fn find_keys(&self, index_name: &str, index_value: &str) -> HashSet<String> {
        self.indexes
            .get(index_name)
            .and_then(|inner_map| inner_map.get(index_value))
            .cloned()
            .unwrap_or_default()
    }
}

pub struct NeoDB {
    pub dbname: String,
    pub collections: HashMap<String, NeoCollection>,
    collection_number: usize,
}

impl NeoDB {
    pub fn new(dbname: &str) -> Self {
        Self {
            dbname: dbname.to_string(),
            collections: HashMap::new(),
            collection_number: 0,
        }
    }

    pub fn collection(&mut self, collection_name: Option<String>) -> &mut NeoCollection {
        let name = match collection_name {
            Some(name) => name,
            None => {
                self.collection_number += 1;
                format!("Collection_{}", self.collection_number)
            }
        };

        self.collections
            .entry(name.clone())
            .or_insert_with(|| NeoCollection::new(name))
    }

    pub fn list_collections(&self) -> Vec<String> {
        self.collections.keys().cloned().collect()
    }

    pub fn drop_collection(&mut self, collection_name: &str) -> bool {
        self.collections.remove(collection_name).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neodb_auto_naming() {
        let mut db = NeoDB::new("test");
        let col1 = db.collection(None);
        assert_eq!(col1.name, "Collection_1");
        let col2 = db.collection(None);
        assert_eq!(col2.name, "Collection_2");
    }

    #[test]
    fn test_neodb_named_collection() {
        let mut db = NeoDB::new("test");
        let col = db.collection(Some("users".to_string()));
        assert_eq!(col.name, "users");
        
        let col_again = db.collection(Some("users".to_string()));
        assert_eq!(col_again.name, "users");
    }

    #[test]
    fn test_neocollection_put_get() {
        let mut col = NeoCollection::new("test".to_string());
        col.put("k1".to_string(), "v1".to_string(), None);
        assert_eq!(col.get("k1"), Some(&"v1".to_string()));
        assert_eq!(col.get("k2"), None);
    }

    #[test]
    fn test_indexing() {
        let mut col = NeoCollection::new("test".to_string());
        let mut indexes = HashMap::new();
        indexes.insert("color".to_string(), "red".to_string());
        
        col.put("k1".to_string(), "v1".to_string(), Some(indexes.clone()));
        col.put("k2".to_string(), "v2".to_string(), Some(indexes));

        let red_keys = col.find_keys("color", "red");
        assert_eq!(red_keys.len(), 2);
        assert!(red_keys.contains("k1"));
        assert!(red_keys.contains("k2"));

        col.delete("k1");
        let red_keys_after = col.find_keys("color", "red");
        assert_eq!(red_keys_after.len(), 1);
        assert!(red_keys_after.contains("k2"));
    }

    #[test]
    fn test_list_and_drop_collections() {
        let mut db = NeoDB::new("test");
        db.collection(Some("c1".to_string()));
        db.collection(Some("c2".to_string()));
        
        let list = db.list_collections();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"c1".to_string()));
        assert!(list.contains(&"c2".to_string()));

        assert!(db.drop_collection("c1"));
        assert!(!db.drop_collection("c1"));
        assert_eq!(db.list_collections().len(), 1);
    }

    #[test]
    fn test_delete_record() {
        let mut col = NeoCollection::new("test".to_string());
        col.put("k1".to_string(), "v1".to_string(), None);
        assert!(col.delete("k1"));
        assert!(!col.delete("k1"));
        assert_eq!(col.get("k1"), None);
    }
}
