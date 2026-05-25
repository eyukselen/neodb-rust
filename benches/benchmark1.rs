use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use neodb::NeoDB;
use std::collections::HashMap;

fn bench_inserts(c: &mut Criterion) {
    // 1. Prepare your data exactly like before
    let mut record_list = Vec::with_capacity(1_000_000);
    for x in 0..1_000_000 {
        let mut indexes = HashMap::new();
        indexes.insert("index-1".to_string(), (x % 10).to_string());
        record_list.push((format!("key-{}", x), format!("value{}", x), indexes));
    }

    // 2. Tell Criterion to measure the execution time of your loop
    c.bench_function("neodb 1m inserts", |b| {
        b.iter(|| {
            let mut db = NeoDB::new("neodb");
            let col = db.collection(None);

            for (k, v, i) in &record_list {
                // Cloning if your API takes owned strings,
                // or just pass references if your API allows it
                col.put(k.clone(), v.clone(), Some(i.clone()));
            }
        })
    });
}

// 3. These macros automatically generate the main entry point for Cargo
criterion_group!(benches, bench_inserts);
criterion_main!(benches);