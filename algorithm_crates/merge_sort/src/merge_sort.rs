use std::cmp::PartialOrd;

fn _merge<T>(x: &mut [T], p: usize, q: usize, r: usize)
where
    T: PartialOrd + Copy,
{
    let left = x[p..q].to_vec();
    let right = x[q..r].to_vec();
    let (mut il, mut ir) = (0, 0);
    for j in p..r {
        let left_value = left.get(il);
        let right_value = right.get(ir);
        match (left_value, right_value) {
            (Some(lv), Some(rv)) => {
                if lv <= rv {
                    x[j] = *lv;
                    il += 1;
                } else {
                    x[j] = *rv;
                    ir += 1;
                }
            }
            (Some(lv), None) => {
                x[j] = *lv;
                il += 1;
            }
            (None, Some(rv)) => {
                x[j] = *rv;
                ir += 1;
            }
            _ => break,
        };
    }
}

fn _merge_sort<T>(x: &mut [T], p: usize, r: usize)
where
    T: PartialOrd + Copy,
{
    if (p + 1) >= r {
        return;
    } else {
        let q = (p + r) / 2;
        _merge_sort(x, p, q);
        _merge_sort(x, q, r);
        _merge(x, p, q, r);
    }
}

pub fn merge_sort<T>(x: &mut [T])
where
    T: PartialOrd + Copy,
{
    let n = x.len();
    _merge_sort(x, 0, n);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut x = vec![1, 3, 4, 4, 5, 2, 4, 5, 6];
        _merge(&mut x, 0, 5, 9);
        assert_eq!(x, vec![1, 2, 3, 4, 4, 4, 5, 5, 6]);
        let mut x = vec![5, 2, 4, 7, 1, 3, 2, 6];
        let n = x.len();
        _merge_sort(&mut x, 0, n);
        assert_eq!(x, vec![1, 2, 2, 3, 4, 5, 6, 7]);

        let mut x = vec![5, 2, 4, 7, 1, 3, 2, -1];
        merge_sort(&mut x);
        assert_eq!(x, vec![-1, 1, 2, 2, 3, 4, 5, 7]);

        let mut x = vec![5, 2, 4, 7, 1, 3, 2];
        merge_sort(&mut x);
        assert_eq!(x, vec![1, 2, 2, 3, 4, 5, 7]);
    }
}
