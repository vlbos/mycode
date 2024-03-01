// 170. Two Sum III - Data structure design
// Design and implement a TwoSum class. It should support the following operations: add and find.

// add - Add the number to an internal data structure.
// find - Find if there exists any pair of numbers which sum is equal to the value.

// Example 1:

// add(1); add(3); add(5);
// find(4) -> true
// find(7) -> false
// Example 2:

// add(3); add(1); add(2);
// find(3) -> true
// find(6) -> false
// Difficulty:
// Easy
// Lock:
// Prime
// Company:
// Facebook LinkedIn

// @lc code=start
use std::collections::HashMap;
pub struct TwoSum {
    // values: Vec<i32>,
    // counts: Vec<usize>,
    cnt: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        // TwoSum {
        //     values: vec![],
        //     counts: vec![],
        // }
        Self {
            cnt: HashMap::new(),
        }
    }

    #[allow(clippy::comparison_chain)]
    /** Add the number to an internal data structure.. */
    pub fn add(&mut self, number: i32) {
        // let mut start = 0;
        // let mut end = self.values.len();
        // while start < end {
        //     let mid = (start + end) / 2;
        //     let mid_value = self.values[mid];
        //     if number < mid_value {
        //         end = mid;
        //     } else if number > mid_value {
        //         start = mid + 1;
        //     } else {
        //         self.counts[mid] += 1;
        //         break;
        //     }
        // }
        // if start == end {
        //     self.values.insert(start, number);
        //     self.counts.insert(start, 1);
        // }
        *self.cnt.entry(number).or_insert(0) += 1;
    }

    #[allow(clippy::comparison_chain)]
    /** Find if there exists any pair of numbers which sum is equal to the value. */
    pub fn find(&self, value: i32) -> bool {
        // let mut i = 0i32;
        // let mut j = (self.values.len() as i32) - 1;
        // while i < j {
        //     let sum = self.values[i as usize] + self.values[j as usize];
        //     if sum == value {
        //         return true;
        //     } else if sum > value {
        //         j -= 1;
        //     } else {
        //         i += 1;
        //     }
        // }
        // i == j && self.counts[i as usize] >= 2usize && self.values[i as usize] * 2 == value
        for (&k, &v) in &self.cnt {
            let remain = value - k;
            if k == remain {
                if v > 1 {
                    return true;
                }
            } else if self.cnt.contains_key(&remain) {
                return true;
            }
        }
        false
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_two_sum_data_structure_1() {
        let mut two_sum = TwoSum::new();
        two_sum.add(1);
        two_sum.add(3);
        two_sum.add(5);
        assert!(two_sum.find(4));
        assert!(!two_sum.find(7));
    }

    #[test]
    pub fn test_two_sum_data_structure_2() {
        let mut two_sum = TwoSum::new();
        two_sum.add(3);
        two_sum.add(1);
        two_sum.add(2);
        assert!(two_sum.find(3));
        assert!(!two_sum.find(6));
    }

    #[test]
    pub fn test_two_sum_data_structure_3() {
        let mut two_sum = TwoSum::new();
        two_sum.add(2);
        two_sum.add(3);
        two_sum.add(2);
        two_sum.add(5);
        assert!(two_sum.find(4));
    }
}
