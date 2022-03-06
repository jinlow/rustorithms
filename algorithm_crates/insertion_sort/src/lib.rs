use std::cmp::PartialOrd;

pub fn insertion_sort<T: PartialOrd + Copy>(x: &mut [T], ascending: bool) {
    let mut key;
    let mut i;
    for j in 1..x.len() {
        key = x[j];
        i = j - 1;
        while if ascending { x[i] > key } else { x[i] < key } {
            x.swap(i + 1, i);
            if i == 0 {
                break;
            }
            i = i - 1;
        }
    }
}

// One iteration where we move 0
// [1, 2, 3, 0, 1]
// [1, 2, 0, 3, 1]
// [1, 0, 2, 3, 1]
// [0, 1, 2, 3, 1]

#[cfg(test)]
mod tests {
    use crate::insertion_sort;

    #[test]
    fn it_works() {
        let mut x = vec![5, 4, 6, 4, 1];
        println!("{:?}", x);
        insertion_sort(&mut x, true);
        assert_eq!(x, vec![1, 4, 4, 5, 6]);
        insertion_sort(&mut x, false);
        assert_eq!(x, vec![6, 5, 4, 4, 1]);
    }
}
