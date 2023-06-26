use once_cell::sync::Lazy;
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

#[inline(always)]
fn hashmap_lookup_inlined(name: &str) -> Option<&u64> {
    HASHMAP.get(name)
}

fn main() {
    println!("Hello, world!");
    let some_names = ["Alice", "Emil", "Xerxes"];
    for _i in 1..1000 {
        for name in some_names {
            let result = hashmap_lookup_inlined(name);
            if let Some(result) = result {
                println!("{name} - {result}");
            }
        }
    }
}
