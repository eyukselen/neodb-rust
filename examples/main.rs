use neodb::NeoDB;

fn main() {
    let mut db = NeoDB::new("test_db");
    
    // Create or get collection
    let col = db.collection(Some("users".to_string()));
    
    // Put record with indexes
    let mut indexes = std::collections::HashMap::new();
    indexes.insert("role".to_string(), "admin".to_string());
    col.put("user1".to_string(), "Alice".to_string(), Some(indexes));
    
    println!("Value for user1: {:?}", col.get("user1"));
    
    // Find by index
    let admins = col.find_keys("role", "admin");
    println!("Admins: {:?}", admins);
    
    // List collections
    println!("Collections: {:?}", db.list_collections());
}

