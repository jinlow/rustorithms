fn main() {
    let mut a1: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
    println!("Unsorted array: {:?}", a1);
    quicksort(&mut a1);
    println!("Sorted array: {:?}", a1);

    // Second implementation
    let mut a1: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
    println!("Unsorted array: {:?}", a1);
    quicksort_mv(&mut a1);
    println!("Sorted array: {:?}", a1);
}

// Pivot the array around the first element.
fn pivot_array<T: std::cmp::Ord>(x: &mut [T]) -> usize {
    let pivot: usize = 0;
    let mut low: usize = 1;
    let mut high: usize = x.len() - 1;
    let max_idx = high;
    while low < high {
        while (x[high] >= x[pivot]) && (high > 0) {
            high -= 1;
        }
        while (x[low] <= x[pivot]) && (low < max_idx) {
            low += 1;
        }
        if low < high {
            x.swap(high, low)
        }
    }
    let pivot = if x[pivot] > x[high] {
        x.swap(pivot, high);
        high
    } else {
        pivot
    };
    pivot
}

// Quicksort implementation
// This is the more efficient implementation unless the input array is already
// sorted.
fn quicksort<T: std::cmp::Ord>(x: &mut [T]) {
    if x.len() < 2 {
        return;
    } else {
        let pivot = pivot_array(x);
        quicksort(&mut x[..pivot]);
        quicksort(&mut x[(pivot + 1)..]);
    }
}

// Move a given element in front of another given element
fn move_front<T>(x: &mut [T], i: usize, j: usize) {
    for k in 0..(j - i) {
        x.swap(j - (k + 1), j - k)
    }
}

// Pivot the array around the first element
// Currently this uses the move front function
// which is likely inefficient.
fn pivot_array_mv<T: std::cmp::Ord>(x: &mut [T]) -> usize {
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
fn quicksort_mv<T: std::cmp::Ord>(x: &mut [T]) {
    if x.len() < 2 {
        return;
    } else {
        let pivot = pivot_array_mv(x);
        quicksort_mv(&mut x[..pivot]);
        quicksort_mv(&mut x[(pivot + 1)..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_of_i32() {
        let mut a1: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
        let a2: [i32; 8] = [-6, -1, 0, 1, 3, 4, 5, 10];
        quicksort(&mut a1);
        assert_eq!(a1, a2)
    }

    #[test]
    fn test_array_2_elements() {
        let mut a1: [u8; 2] = [5, 4];
        let a2: [u8; 2] = [4, 5];
        quicksort(&mut a1);
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_compare_quick_sorts() {
        let mut a1: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
        let mut a2: [i32; 8] = [3, 1, 4, 5, 10, -1, -6, 0];
        quicksort(&mut a1);
        quicksort_mv(&mut a2);
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_string_bytes() {
        let s_sorted = String::from("  ASTbeeghhimmnoottu").into_bytes();
        let mut s_unsorted = String::from("Seomthing About Them").into_bytes();
        quicksort(&mut s_unsorted);
        assert_eq!(s_sorted, s_unsorted);
    }
}
