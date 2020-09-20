use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let a = 10;
    let b = "10";
    let c = "10";
    println!(
        "Num Hash {}, str Hash {}, Other {}",
        calc_hash(&a),
        calc_hash(&b),
        calc_hash(&c)
    );
}

// impl calc_hash for Num {
//     1
// }

fn calc_hash<T: Hash>(x: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}
