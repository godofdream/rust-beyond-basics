use mimalloc::MiMalloc;
use std::collections::HashMap;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let mut map = HashMap::new();

    for i in 0..100 {
        map.insert(i, format!("{i}*10 = {}", i*10));
    }

    println!("{:?}", map);
}
