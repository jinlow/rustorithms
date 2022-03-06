use merge_sort::merge_sort;

fn main() {
    let mut x = vec![5, 2, 4, 7, 1, 3, 2];
    println!("Pre-sort: {:?}", x);
    merge_sort(&mut x);
    println!("After-sort: {:?}", x);
}
