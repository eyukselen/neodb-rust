use criterion::{criterion_group, criterion_main, Criterion};
use neodb::NeoDB;
use std::collections::HashMap;

fn bench_inserts(c: &mut Criterion) {
    let mut record_list = Vec::with_capacity(1_000_000);
    for x in 0..1_000_000 {
        let mut indexes = HashMap::new();
        indexes.insert("index-1".to_string(), (x % 10).to_string());
        record_list.push((format!("key-{}", x), format!("value{}", x), indexes));
    }

    c.bench_function("neodb 1m inserts", |b| {
        b.iter(|| {
            let mut db = NeoDB::new("neodb");
            let col = db.collection(None);

            for (k, v, i) in &record_list {
                col.put(k.clone(), v.clone(), Some(i.clone()));
            }
        })
    });
}

criterion_group!(benches, bench_inserts);
criterion_main!(benches);