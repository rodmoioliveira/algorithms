// REFERENCES:
//
// - [Wild2012] S. Wild. Java 7's Dual Pivot Quicksort. Master's Thesis Department of Computer Science, Technische Universität Kaiserslautern, 2012.
// - [Wild2016] S. Wild. Dual-Pivot Quicksort and Beyond: Analysis of Multiway Partitioning and Its Practical Potential. Doctoral's Thesis Department of Computer Science, Technische Universität Kaiserslautern, 2016.
// - [Yar09] V. Yaroslavskiy, 2009. Dual-Pivot Quicksort. URL: https://codeblab.com/wp-content/uploads/2009/09/DualPivotQuicksort.pdf
// - https://learnforeverlearn.com/yaro_web/
// - https://www.geeksforgeeks.org/dual-pivot-quicksort/
//
// [Wild2016, p.139]
// Since Java 7, the reference implementation of the Java runtime library uses dual-pivot Quicksort
// as default sorting method for primitive-type arrays. At its core is the ternary partitioning
// method given in Algorithm 4 below. The algorithm is due to V. Yaroslavskiy, J. Bentley and J.
// Bloch and will be referred to as YBB Quicksort for short. I previously called it Yaroslavskiy's
// algorithm, because it was him who first discovered that time is ripe for a Quicksort with two
// pivots. From personal communication with the three I learned that YBB Quicksort is more
// appropriate since Bentley and Bloch were involved in early stages of the development of the
// algorithm.
//
// [Wild2016, p.140]
//
//   ┄ ┄┌──────────────────────┬──────────────────────┬────────────────────┬──────────────────────┐┄ ┄
//      │        < P1          │    P1 <= o <= P2     │         ?          │         >= P2        │
//   ┄ ┄└──────────────────────┴──────────────────────┴────────────────────┴──────────────────────┘┄ ┄
//    left                     j                      k                    g                    right
//                            -->                    -->                  <--
//
pub fn _dual_pivot_yaroslavskiy_bentley_bloch<T: Copy + Ord>(
    a: &mut [T],
    left: usize,
    right: usize,
) {
    if left < right {
        if a[left] > a[right] {
            a.swap(left, right);
        }

        // pointers
        let mut j = left + 1;
        let mut g = right - 1;
        let mut k = left + 1;

        // pivot_l is the left pivot, and pivot_r is the right pivot.
        let pivot_l = a[left];
        let pivot_r = a[right];

        while k <= g {
            // If elements are less than the left pivot
            if a[k] < pivot_l {
                a.swap(k, j);
                j += 1;
            }
            // If elements are greater than or equal to the right pivot
            else if a[k] >= pivot_r {
                while a[g] > pivot_r && k < g {
                    g -= 1;
                }
                a.swap(k, g);
                g -= 1;
                if a[k] < pivot_l {
                    a.swap(k, j);
                    j += 1;
                }
            }
            k += 1;
        }
        j -= 1;
        g += 1;

        // Bring pivots to their appropriate positions.
        a.swap(left, j);
        a.swap(right, g);

        if j > 0 {
            _dual_pivot_yaroslavskiy_bentley_bloch(a, left, j - 1);
        }
        _dual_pivot_yaroslavskiy_bentley_bloch(a, j + 1, g - 1);
        _dual_pivot_yaroslavskiy_bentley_bloch(a, g + 1, right);
    }
}

pub fn dual_pivot_yaroslavskiy_bentley_bloch<T: Copy + Ord>(a: &mut [T]) {
    let len = a.len();
    if len > 1 {
        _dual_pivot_yaroslavskiy_bentley_bloch(a, 0, len - 1);
    }
}

pub fn example() {
    let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
    dual_pivot_yaroslavskiy_bentley_bloch(&mut res);
    eprintln!("{:?} {}", res, module_path!());
}

#[cfg(test)]
mod tests {
    use super::dual_pivot_yaroslavskiy_bentley_bloch;
    use crate::sorting::utils::*;

    #[test]
    fn basic() {
        let mut res = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let cloned = res.clone();
        dual_pivot_yaroslavskiy_bentley_bloch(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        dual_pivot_yaroslavskiy_bentley_bloch(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        dual_pivot_yaroslavskiy_bentley_bloch(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        dual_pivot_yaroslavskiy_bentley_bloch(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        dual_pivot_yaroslavskiy_bentley_bloch(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        dual_pivot_yaroslavskiy_bentley_bloch(&mut res);

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        log_timed("large elements test", || {
            dual_pivot_yaroslavskiy_bentley_bloch(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();
        log_timed("nearly ordered elements test", || {
            dual_pivot_yaroslavskiy_bentley_bloch(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = generate_repeated_elements_vec(1_000, 3);
        let cloned = res.clone();
        log_timed("repeated elements test", || {
            dual_pivot_yaroslavskiy_bentley_bloch(&mut res);
        });

        assert!(is_sorted(&res));
        assert!(have_same_elements(&res, &cloned));
    }
}
