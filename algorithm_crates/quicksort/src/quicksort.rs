// Pivot the array around the first element.
fn _pivot_array<T: std::cmp::Ord>(x: &mut [T]) -> usize {
    let pivot: usize = 0;
    let mut low: usize = 1;
    let mut high: usize = x.len() - 1;

    // Select median-of-three as pivot
    let mid: usize = high / 2;
    if x[pivot] > x[mid] {
        x.swap(pivot, mid)
    }
    if x[high] < x[pivot] {
        x.swap(high, pivot);
    }
    if x[mid] > x[high] {
        x.swap(mid, high);
    }
    x.swap(pivot, mid);

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
// This utilizes the median of three approach to help
// reduce the chance of selecting a bad pivot value.
pub fn quicksort<T: std::cmp::Ord>(x: &mut [T]) {
    if x.len() < 2 {
        return;
    } else {
        let pivot = _pivot_array(x);
        quicksort(&mut x[..pivot]);
        quicksort(&mut x[(pivot + 1)..]);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
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
    fn test_qs_on_sorted() {
        let mut a1: Vec<i32> = (0..10000).collect();
        let a2: Vec<i32> = (0..10000).collect();
        quicksort(&mut a1);
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_random_vector() {
        let mut rng = rand::thread_rng();
        let mut a1: Vec<i32> = (0..10000).map(|_| rng.gen_range(0..10000)).collect();
        let mut a2 = a1.to_vec();
        quicksort(&mut a1);
        a2.sort();
        assert_eq!(a1, a2);
    }

    #[test]
    fn test_string_bytes() {
        let s_sorted = String::from("  ASTbeeghhimmnoottu").into_bytes();
        let mut s_unsorted = String::from("Something About Them").into_bytes();
        quicksort(&mut s_unsorted);
        assert_eq!(s_sorted, s_unsorted);
    }
}
