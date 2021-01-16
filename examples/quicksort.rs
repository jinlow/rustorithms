use quicksort::quicksort;

fn main() {
    let mut a1: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
    println!("Unsorted array: {:?}", a1);
    quicksort(&mut a1);
    println!("Sorted array: {:?}", a1);
}