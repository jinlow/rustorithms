use binary_search::binary_search;

fn main() {
    let ar: [i32; 10] = [1, 2, 43, 70, 100, 200, 300, 500, 600, 1000];
    println!("{}\n", binary_search(&ar, 70).unwrap());
    let ar2: Vec<i32> = (0..1000000).collect();
    println!("{}", binary_search(&ar2, 70).unwrap());
}