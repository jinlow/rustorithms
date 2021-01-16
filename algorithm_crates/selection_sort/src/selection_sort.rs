/// Find the smallest element in a slice.
pub fn find_smallest<T: std::cmp::Ord>(a: &[T]) -> usize {
    let mut smallest: usize = 0;
    for i in 0..a.len() {
        smallest = if &a[smallest] < &a[i] { smallest } else { i };
    }
    smallest
}

/// Selection sort implementation.
pub fn selection_sort<T: std::cmp::Ord>(a: &mut [T]) {
    let mut smallest: usize;
    for i in 0..a.len() {
        smallest = find_smallest(&a[i..]);
        a.swap(i, smallest + i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_selection_sort_int() {
        let mut x = [0, 100, 5, 6, -2];
        selection_sort(&mut x);
        assert_eq!(x, [-2, 0, 5, 6, 100]);
    }

    #[test]
    fn test_selection_sort_string() {
        let s_sorted = String::from("  ASTbeeghhimmnoottu").into_bytes();
        let mut s_unsorted = String::from("Something About Them").into_bytes();
        selection_sort(&mut s_unsorted);
        assert_eq!(s_sorted, s_unsorted);
    }

    #[test]
    fn test_selection_sort_str() {
        let mut x = ["0", "100", "5", "6", "-2"];
        selection_sort(&mut x);
        assert_eq!(x, ["-2", "0", "100", "5", "6"]);
    }
}
