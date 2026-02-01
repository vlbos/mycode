// # 3672. Sum of Weighted Modes in Subarrays 🔒

// Description
// -----------

// You are given an integer array `nums` and an integer `k`.

// For every **subarray** of length `k`:

// *   The **mode** is defined as the element with the **highest frequency**. If there are multiple choices for a mode, the **smallest** such element is taken.
// *   The **weight** is defined as `mode * frequency(mode)`.

// Return the **sum** of the weights of all **subarrays** of length `k`.

// **Note:**

// *   A **subarray** is a contiguous **non-empty** sequence of elements within an array.
// *   The **frequency** of an element `x` is the number of times it occurs in the array.

// **Example 1:**

// **Input:** nums = \[1,2,2,3\], k = 3

// **Output:** 8

// **Explanation:**

// Subarrays of length `k = 3` are:

// |Subarray |Frequencies|Mode|Mode ​​​​​​​Frequency|Weight   |
// |---------|-----------|----|---------------------|---------|
// |[1, 2, 2]|1: 1, 2: 2 |2   |2                    |2 × 2 = 4|
// |[2, 2, 3]|2: 2, 3: 1 |2   |2                    |2 × 2 = 4|

// Thus, the sum of weights is `4 + 4 = 8`.

// **Example 2:**

// **Input:** nums = \[1,2,1,2\], k = 2

// **Output:** 3

// **Explanation:**

// Subarrays of length `k = 2` are:

// |Subarray|Frequencies|Mode|Mode Frequency|Weight   |
// |--------|-----------|----|--------------|---------|
// |[1, 2]  |1: 1, 2: 1 |1   |1             |1 × 1 = 1|
// |[2, 1]  |2: 1, 1: 1 |1   |1             |1 × 1 = 1|
// |[1, 2]  |1: 1, 2: 1 |1   |1             |1 × 1 = 1|

// Thus, the sum of weights is `1 + 1 + 1 = 3`.

// **Example 3:**

// **Input:** nums = \[4,3,4,3\], k = 3

// **Output:** 14

// **Explanation:**

// Subarrays of length `k = 3` are:

// |Subarray |Frequencies|Mode|Mode Frequency|Weight   |
// |---------|-----------|----|--------------|---------|
// |[4, 3, 4]|4: 2, 3: 1 |4   |2             |2 × 4 = 8|
// |[3, 4, 3]|3: 2, 4: 1 |3   |2             |2 × 3 = 6|

// Thus, the sum of weights is `8 + 6 = 14`.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `1 <= nums[i] <= 105`
// *   `1 <= k <= nums.length`

// // long long mode_weight(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn mode_weight(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::{BinaryHeap, HashMap};
        let mut q = BinaryHeap::new();
        let mut cnt = HashMap::new();
        let k = k as usize;
        for &x in &nums[..k] {
            *cnt.entry(x).or_insert(0) += 1;
            q.push((cnt[&x], -x));
        }
        let mut get_mode = |cnt: &HashMap<i32, i32>, q: &mut BinaryHeap<(i32, i32)>| {
            while q.peek().is_some_and(|v| v.0 != cnt[&-v.1]) {
                q.pop();
            }
            let &(freq, val) = q.peek().unwrap();
            freq as i64 * -val as i64
        };
        let mut ans = get_mode(&cnt, &mut q);
        for (i, &x) in nums.iter().enumerate().skip(k) {
            let y = nums[i - k];
            *cnt.entry(x).or_insert(0) += 1;
            *cnt.entry(y).or_insert(0) -= 1;
            q.push((cnt[&x], -x));
            q.push((cnt[&y], -y));
            ans += get_mode(&cnt, &mut q);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_mode_weight_1() {
        assert_eq!(8, Solution::mode_weight(vec![1, 2, 2, 3], 3));
    }
    #[test]
    pub fn test_mode_weight_2() {
        assert_eq!(3, Solution::mode_weight(vec![1, 2, 1, 2], 2));
    }
    #[test]
    pub fn test_mode_weight_3() {
        assert_eq!(14, Solution::mode_weight(vec![4, 3, 4, 3], 3));
    }
}
