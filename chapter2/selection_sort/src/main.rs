fn main() {
    let v1: [i32; 5] = [10, 4, 10, 2, -1];
    println!("The smallest value is {}", &v1[find_smallest(&v1)]);
    println!("The array is {:?}", v1);
    let mut v2 = vec![50, 40, 1, 30, 11, -1, -1];
    println!("The array is        {:?}", v2);
    selection_sort(&mut v2);
    println!("The sorted array is {:?}:", v2);
}

fn find_smallest<T: std::cmp::Ord>(a: &[T]) -> usize {
    let mut smallest: usize = 0;
    for i in 0..a.len() {
        smallest = if &a[smallest] < &a[i] { smallest } else { i };
    }
    smallest
}

fn selection_sort<T: std::cmp::Ord>(a: &mut [T]) {
    let mut smallest: usize;
    for i in 0..a.len() {
        smallest = find_smallest(&a[i..]);
        a.swap(i, smallest + i);
    }
}
