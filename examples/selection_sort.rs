use selection_sort::{find_smallest, selection_sort};

fn main() {
    let v1: [i32; 5] = [10, 4, 10, 2, -1];
    println!("The smallest value is {}", &v1[find_smallest(&v1)]);
    println!("The array is {:?}", v1);
    let mut v2 = vec![50, 40, 1, 30, 11, -1, -1];
    println!("The array is        {:?}", v2);
    selection_sort(&mut v2);
    println!("The sorted array is {:?}:", v2);
}
