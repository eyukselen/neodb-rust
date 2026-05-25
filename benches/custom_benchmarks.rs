use neodb::NeoDB;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let mut db = neodb::NeoDB::new("neodb");
    let col = db.collection(None);

    let mut record_list = Vec::with_capacity(1_000_000);
    for x in 0..1_000_000 {
        let mut indexes = HashMap::new();
        indexes.insert("index-1".to_string(), (x % 10).to_string());
        record_list.push((format!("key-{}", x), format!("value{}", x), indexes));
    }

    let start_time = Instant::now();
    for (k, v, i) in record_list {
        col.put(k, v, Some(i));
    }
    let duration = start_time.elapsed();

    println!("duration is:{:?}", duration.as_secs_f64());
}
