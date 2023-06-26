use criterion::{black_box, criterion_group, criterion_main, Criterion};
use once_cell::sync::Lazy;
use phf::phf_map;
use std::collections::HashMap;

/// A Hashmap lazyly initialized, globally accessable
static HASHMAP: Lazy<HashMap<String, u64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Alice".to_string(), 25);
    map.insert("Bob".to_string(), 30);
    map.insert("Charlie".to_string(), 35);
    map.insert("Dennis".to_string(), 35);
    map.insert("Emil".to_string(), 35);
    map.insert("Fabian".to_string(), 35);
    map
});

/// A precompiled Hashmap with a perfect Hash function
static PHASHMAP: phf::Map<&'static str, u64> = phf_map! {
    "Alice" => 25,
    "Bob" => 30,
    "Charlie" => 35,
    "Dennis" => 35,
    "Emil" => 35,
    "Fabian" => 35,
};

#[inline(never)]
fn hashmap_lookup_not_inlined(name: &str) -> Option<&u64> {
    HASHMAP.get(name)
}

#[inline(always)]
fn hashmap_lookup_inlined(name: &str) -> Option<&u64> {
    HASHMAP.get(name)
}

#[inline(always)]
fn phashmap_lookup_inlined(name: &str) -> Option<&u64> {
    PHASHMAP.get(name)
}

#[allow(clippy::unit_arg)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hashmap not inlined", |b| {
        // Optionally include some setup
        let some_names = ["Alice", "Emil", "Xerxes"];
        b.iter(|| {
            // Inner closure, the actual test
            for name in some_names {
                black_box({
                    let _result = hashmap_lookup_not_inlined(name);
                }); // black_box prevents the optimizer to inline the for loop
            }
        });
    });

    c.bench_function("hashmap inlined", |b| {
        // Optionally include some setup
        let some_names = ["Alice", "Emil", "Xerxes"];
        b.iter(|| {
            // Inner closure, the actual test
            for name in some_names {
                black_box({
                    let _result = hashmap_lookup_inlined(name);
                }); // black_box prevents the optimizer to inline the for loop
            }
        });
    });

    c.bench_function("phashmap inlined", |b| {
        // Optionally include some setup
        let some_names = ["Alice", "Emil", "Xerxes"];
        b.iter(|| {
            // Inner closure, the actual test
            for name in some_names {
                black_box({
                    let _result = phashmap_lookup_inlined(name);
                }); // black_box prevents the optimizer to inline the for loop
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
