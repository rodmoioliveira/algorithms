// https://www.geeksforgeeks.org/find-missing-element-in-a-sorted-array-of-consecutive-numbers/
// ================================
// Principles:
// ================================

// Look for inconsistency: Ideally, the difference between any element and its index must be arr[0]
// for every element.

// Example,
// A[] = {1, 2, 3, 4, 5} -> Consistent
// B[] = {101, 102, 103, 104} -> Consistent
// C[] = {1, 2, 4, 5, 6} -> Inconsistent as C[2] – 2 != C[0] i.e. 4 – 2 != 1

// Finding inconsistency helps to scan only half of the array each time in O(logN).

// ================================
// Algorithm
// ================================

// Find middle element and check if it’s consistent.

// If middle element is consistent, then check if the difference between middle element and its
// next element is greater than 1 i.e. check if arr[mid + 1] – arr[mid] > 1

// If yes, then arr[mid] + 1 is the missing element.

// If not, then we have to scan the right half array from the middle element and jump to step-1.

// If middle element is inconsistent, then check if the difference between middle element and its
// previous element is greater than 1 i.e. check if arr[mid] – arr[mid – 1] > 1

// If yes, then arr[mid] – 1 is the missing element.

// If not, then we have to scan the left half array from the middle element and jump to step-1.
pub fn missing(array: &[i64], len: usize) -> Option<i64> {
    let mut l_idx = 0;
    let mut r_idx = len as i64 - 1;
    let first_el = array[0];

    while r_idx > l_idx {
        let m_idx = (l_idx + r_idx) / 2;
        let m_idx_usize = m_idx as usize;
        let m_is_consistent = array[m_idx_usize] - m_idx == first_el;

        // Check if middle element is consistent
        if m_is_consistent {
            let next_is_consistent = array[m_idx_usize + 1] - array[m_idx_usize] == 1;

            // When missing element is just after the middle element
            if next_is_consistent {
                l_idx = m_idx + 1;
            } else {
                return Some(array[m_idx_usize] + 1);
            }
        } else {
            let prev_is_consistent = array[m_idx_usize] - array[m_idx_usize - 1] == 1;

            // When missing element is just before the middle element
            if prev_is_consistent {
                r_idx = m_idx - 1;
            } else {
                return Some(array[m_idx_usize] - 1);
            }
        }
    }

    None
}

// https://en.wikipedia.org/wiki/Binary_search_algorithm#Procedure
pub fn iteractive(array: &[i64], len: usize, target: i64) -> Option<usize> {
    let mut l_idx = 0;
    let mut r_idx = len as i64 - 1;

    while l_idx <= r_idx {
        let m_idx = (l_idx + r_idx) / 2;
        let m_idx_usize = m_idx as usize;
        let mid_val = array[m_idx_usize];

        if mid_val == target {
            return Some(m_idx_usize);
        };

        if mid_val < target {
            l_idx = m_idx + 1;
        }

        if mid_val > target {
            r_idx = m_idx - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing() {
        let mut array: Vec<i64> = (0..=1).collect();
        let mut array1: Vec<i64> = (3..=100_000).collect();
        array.append(&mut array1);

        assert_eq!(missing(&array, array.len()), Some(2));
    }

    #[test]
    fn test_iteractive() {
        let array: Vec<i64> = (0..=100_000).collect();

        assert_eq!(iteractive(&array, array.len(), 1), Some(1));
        assert_eq!(iteractive(&array, array.len(), 87_304), Some(87_304));
        assert_eq!(iteractive(&array, array.len(), 100_000), Some(100_000));
        assert_eq!(iteractive(&array, array.len(), 100_001), None);
        assert_eq!(iteractive(&array, array.len(), -2), None);
    }
}
