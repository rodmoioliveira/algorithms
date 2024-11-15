// REFERENCES:
//
// - [BM93] J. L. J. Bentley and M. D. McIlroy, 1993. Engineering a sort function. Software: Practice and Experience, 23(11):1249–1265. doi:10.1002/spe.4380231105.
// - https://medium.com/@mcguire.crsr/quicksort-c850a4c6e47 [PERFECT!]
// - https://sedgewick.io/wp-content/uploads/2022/03/2002QuicksortIsOptimal.pdf
// - https://stackoverflow.com/questions/7264101/implementing-bentley-mcilroy-three-way-partitioning-using-stl-iterators
// - https://www.cs.princeton.edu/courses/archive/spring20/cos226/demos/23DemoPartitioning.pdf

// [Wild2012, p.37]
// Algorithm 4. Quicksort with BENTLEY and MCILROY’s three-way partitioning method proposed in
// [BM93].
pub fn _three_way_bentley_mcilroy<T: Ord>(a: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot_i = left;

        let mut i = left;
        let mut j = right;

        let mut p = left;
        let mut q = right;

        loop {
            while i < j && a[i] <= a[pivot_i] {
                if a[i] == a[pivot_i] {
                    a.swap(i, p);
                    p += 1;
                }
                i += 1;
            }
            while i < j && a[pivot_i] <= a[j] {
                if a[j] == a[pivot_i] {
                    a.swap(j, q);
                    q -= 1;
                }
                j -= 1;
            }
            if i >= j {
                break;
            }
            a.swap(i, j);
        }

        if a[i] <= a[pivot_i] && i < right {
            i += 1;
        }
        if a[pivot_i] <= a[j] && left < j {
            j -= 1;
        }

        if p <= j {
            while left < p {
                p -= 1;
                a.swap(p, j);
                j -= 1;
            }
        } else {
            j = left
        }

        if i <= q {
            while q < right {
                q += 1;
                a.swap(q, i);
                i += 1;
            }
        } else {
            i = right
        }

        _three_way_bentley_mcilroy(a, left, j);
        _three_way_bentley_mcilroy(a, i, right);
    }
}

pub fn three_way_bentley_mcilroy<T: Ord>(a: &mut [T]) {
    let len = a.len();
    if len > 1 {
        _three_way_bentley_mcilroy(a, 0, len - 1);
    }
}

pub fn example() {
    let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
    three_way_bentley_mcilroy(&mut res);
    eprintln!("{res:?} sorting::quicksort::three_way_bentley_mcilroy");
}

#[cfg(test)]
mod tests {
    use super::three_way_bentley_mcilroy;
    use crate::sorting::utils::*;

    #[test]
    fn basic() {
        let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let cloned = res.clone();
        three_way_bentley_mcilroy(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        three_way_bentley_mcilroy(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        three_way_bentley_mcilroy(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        three_way_bentley_mcilroy(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        three_way_bentley_mcilroy(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        three_way_bentley_mcilroy(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        log_timed("large elements test", || {
            three_way_bentley_mcilroy(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();
        log_timed("nearly ordered elements test", || {
            three_way_bentley_mcilroy(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1_000_000, 3);
        let cloned = res.clone();
        log_timed("repeated elements test", || {
            three_way_bentley_mcilroy(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }
}
