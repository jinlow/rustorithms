use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let a = 10;
    let b = String::from("10");
    let c = "10";
    println!(
        "Num Hash {}, str Hash {}, Other {}",
        calc_hash(&a),
        calc_hash(&b),
        calc_hash(&c),
    );

    let mut ht = Hashtable::<i32, String>::new();
    ht.add(10, String::from("10"));
    ht.add(11, String::from("11"));
    ht.print_ht();
    println!("Key 10, data: {}", ht.get(10).unwrap());
    ht.delete(11).unwrap();
    ht.print_ht();
    ht.add(10, String::from("50"));
    ht.add(30, String::from("30"));
    ht.add(50, String::from("50"));
    ht.add(100, String::from("100"));
    ht.print_ht();
}

// A Hashtable -
// This is a hashtable, modeled after the dictionaries in CPython.
// The biggest difference is the hashing algorithm is different in CPython,
// whereas I am just using the standard hash algorithm right from the Rust
// standard library.
struct Hashtable<K, V> {
    size: usize,
    sparse_key: Vec<Option<i64>>,
    data: Vec<Option<Box<(u64, K, V)>>>,
}

impl<K, V> Hashtable<K, V>
where
    K: std::cmp::Eq + Hash + std::fmt::Display,
    V: std::cmp::Eq + std::fmt::Display,
{
    pub fn new() -> Self {
        Hashtable {
            size: 8,
            sparse_key: (0..8).map(|_| None).collect(),
            data: Vec::new(),
        }
    }

    fn _get_index(&self, hash_value: &u64) -> usize {
        (*hash_value as usize) & (self.size - 2)
    }

    // This function resolves if the index returned from
    // _get_index is full, using the probe function.
    fn _get_final_index(&self, hash_value: &u64, key: &K) -> usize {
        let idx = self._get_index(hash_value);
        let pos = self.sparse_key[idx];
        match pos {
            Some(p) => {
                if p >= 0 {
                    let val = self.data[p as usize].as_ref().unwrap();
                    if (*hash_value == val.0) && (*key == val.1) {
                        return idx;
                    } else {
                        return self._probe_from_hash_key(&hash_value, &key);
                    }
                } else {
                    return self._probe_from_hash_key(&hash_value, &key);
                }
            }
            None => return usize::from(idx),
        }
    }

    // Private function to probe for an index when the first one returned
    // from _get_index is being used.
    fn _probe_from_hash_key(&self, hash_value: &u64, key: &K) -> usize {
        let mask = self.size - 1;
        let mut perturb = *hash_value as usize;
        let mut i = self._get_index(&hash_value);
        loop {
            let pos = self.sparse_key[i];
            match pos {
                Some(p) => {
                    if p >= 0 {
                        // println!("Gotten this far");
                        let val = self.data[p as usize].as_ref().unwrap();
                        if (*hash_value == val.0) && (*key == val.1) {
                            return i;
                        }
                    }
                }
                None => return i,
            }
            perturb = perturb >> 5;
            i = mask & ((i * 5) + perturb + 1);
        }
    }

    // Add a new (key, value) pair to the hashtable
    pub fn add(&mut self, key: K, value: V) {
        if (self.data.len() + 1) >= (((2 * self.size) / 3) as usize) {
            self._doublesize();
        }
        let hash_value = calc_hash(&key);
        self._add_from_hash(hash_value, key, value);
    }

    fn _add_from_hash(&mut self, hash_value: u64, key: K, value: V) {
        let idx = self._get_final_index(&hash_value, &key);
        let pos = self.sparse_key[idx];
        match pos {
            Some(p) => self.data[p as usize] = Some(Box::new((hash_value, key, value))),
            None => {
                self.sparse_key[idx] = Some(self.data.len() as i64);
                self.data.push(Some(Box::new((hash_value, key, value))));
            }
        }
    }

    fn _doublesize(&mut self) {
        // println!("We've doubled");
        self.size *= 2;
        self.sparse_key = (0..self.size).map(|_| None).collect();
        for slot in 0..self.data.len() {
            let dt = &self.data[slot];
            if let Some(d) = dt {
                let idx = self._get_final_index(&d.0, &d.1);
                self.sparse_key[idx] = Some(slot as i64);
            }
        }
    }

    // get item from the hashtable
    pub fn get(&self, key: K) -> Option<&V> {
        let hash_value = calc_hash(&key);
        let idx = self._get_final_index(&hash_value, &key);
        let pos = self.sparse_key[idx];
        match pos {
            Some(p) => {
                let val = self.data[p as usize].as_ref().unwrap();
                Some(&val.2)
            }
            None => return None,
        }
    }

    // Delete an item in the hashtable
    pub fn delete(&mut self, key: K) -> Result<(), &str> {
        let hash_value = calc_hash(&key);
        let idx = self._get_final_index(&hash_value, &key);
        let pos = self.sparse_key[idx];
        match pos {
            Some(p) => {
                self.data[p as usize] = None;
                self.sparse_key[idx] = Some(-1);
                Ok(())
            }
            None => Err("Key not found"),
        }
    }

    // print the contents of the hashtable
    pub fn print_ht(&self) {
        println!("Hashtable Data");
        for i in self.data.iter() {
            if let Some(d) = i {
                println!("{}, {}", d.1, d.2)
            }
        }
    }
}

fn calc_hash<T: Hash>(x: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_keys() {
        let mut ht = Hashtable::<i32, i32>::new();
        for k in 0..1000 {
            ht.add(k, k);
        }
        let k_ord: Vec<i32> = (0..1000).map(|i| i).collect();
        let keys: Vec<i32> = ht.data.iter().map(|x| x.as_ref().unwrap().1).collect();
        assert_eq!(k_ord, keys);
    }

    #[test]
    fn test_get_values() {
        let mut ht = Hashtable::<i32, i32>::new();
        for k in 0..1000 {
            ht.add(k, k);
        }
        let k_ord: Vec<i32> = (0..1000).map(|i| i).collect();
        let values: Vec<i32> = (0..1000).map(|k| *ht.get(k).unwrap()).collect();
        assert_eq!(k_ord, values);
    }
    #[test]
    fn test_delete_first10() {
        let mut ht = Hashtable::<i32, i32>::new();
        for k in 0..1000 {
            ht.add(k, k);
        }
        for k in 0..10 {
            ht.delete(k).unwrap();
        }
        let k_ord: Vec<i32> = (10..1000).map(|i| i).collect();
        let keys: Vec<i32> = ht
            .data
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().1)
            .collect();
        assert_eq!(k_ord, keys);
    }
    #[test]
    fn test_delete_last10() {
        let mut ht = Hashtable::<i32, i32>::new();
        for k in 0..1000 {
            ht.add(k, k);
        }
        for k in 990..1000 {
            ht.delete(k).unwrap();
        }
        let k_ord: Vec<i32> = (0..990).map(|i| i).collect();
        let keys: Vec<i32> = ht
            .data
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().1)
            .collect();
        assert_eq!(k_ord, keys);
    }
}
