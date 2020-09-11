fn main() {
    let mut a1: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
    println!("{:?}", a1);
    quicksort(&mut a1);
    println!("{:?}", a1);
    let mut a2 = vec![4, 3, 2, 1];
    quicksort(&mut a2);
    println!("{:?}", a2);
}

// Move a given element in front of another given element
fn move_front<T>(x: &mut [T], i: usize, j: usize) {
    for k in 0..(j - i) {
        x.swap(j - (k + 1), j - k)
    }
}

// Pivot the array around the first element
// Currently this uses the move infront function
// which is likely inefficient.
fn pivot_array<T: std::cmp::Ord>(x: &mut [T]) -> usize {
    let mut pivot: usize = 0;
    for i in 1..(x.len()) {
        if x[i] < x[pivot] {
            move_front(x, pivot, i);
            pivot = pivot + 1;
        }
    }
    pivot
}

// Quicksort implementation
fn quicksort<T: std::cmp::Ord>(x: &mut [T]) {
    if x.len() < 2 {
        return;
    } else {
        let pivot = pivot_array(x);
        quicksort(&mut x[..pivot]);
        quicksort(&mut x[(pivot + 1)..]);
    }
}
