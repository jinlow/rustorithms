use insertion_sort::insertion_sort;

fn main() {
    let mut x = vec![5, 2, 4, 7, 1, 3, 2];
    println!("Pre-sort: {:?}", x);
    insertion_sort(&mut x, true);
    println!("After-sort: {:?}", x);
}
