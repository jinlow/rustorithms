use hashtable::{Hashtable, calc_hash};


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
