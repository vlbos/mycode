// [3299\. Sum of Consecutive Subsequences 🔒](https://leetcode.com/problems/sum-of-consecutive-subsequences)
// ==========================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// We call an array `arr` of length `n` **consecutive** if one of the following holds:

// *   `arr[i] - arr[i - 1] == 1` for _all_ `1 <= i < n`.
// *   `arr[i] - arr[i - 1] == -1` for _all_ `1 <= i < n`.

// The **value** of an array is the sum of its elements.

// For example, `[3, 4, 5]` is a consecutive array of value 12 and `[9, 8]` is another of value 17. While `[3, 4, 3]` and `[8, 6]` are not consecutive.

// Given an array of integers `nums`, return the _sum_ of the **values** of all **consecutive** _non-empty_ subsequences.

// Since the answer may be very large, return it **modulo** `109 + 7.`

// **Note** that an array of length 1 is also considered consecutive.

// **Example 1:**

// **Input:** nums = \[1,2\]

// **Output:** 6

// **Explanation:**

// The consecutive subsequences are: `[1]`, `[2]`, `[1, 2]`.

// **Example 2:**

// **Input:** nums = \[1,4,2,3\]

// **Output:** 31

// **Explanation:**

// The consecutive subsequences are: `[1]`, `[4]`, `[2]`, `[3]`, `[1, 2]`, `[2, 3]`, `[4, 3]`, `[1, 2, 3]`.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `1 <= nums[i] <= 105`

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn get_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let calc = |nums: &[i32]| {
            let (mut left, mut right) = (vec![0; n], vec![0; n]);
            let mut cnt = std::collections::HashMap::new();
            for i in 1..n {
                *cnt.entry(nums[i - 1]).or_insert(0) +=
                    1 + *cnt.get(&(nums[i - 1] - 1)).unwrap_or(&0);
                left[i] = *cnt.get(&(nums[i] - 1)).unwrap_or(&0);
            }
            cnt.clear();
            for i in (0..n - 1).rev() {
                *cnt.entry(nums[i + 1]).or_insert(0) +=
                    1 + *cnt.get(&(nums[i + 1] + 1)).unwrap_or(&0);
                right[i] = *cnt.get(&(nums[i] + 1)).unwrap_or(&0);
            }
            let modu = 1_000_000_007;
            left.into_iter()
                .zip(right)
                .zip(nums)
                .fold(0, |s, ((l, r), &x)| {
                    (s + (l + r + l * r % modu) * x as i64 % modu) % modu
                })
        };
        let x = calc(&nums);
        nums.reverse();
        let y = calc(&nums);
        let s = nums.into_iter().map(Into::<i64>::into).sum::<i64>();
        ((s + x + y) % 1_000_000_007) as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_get_sum_1() {
        assert_eq!(6, Solution::get_sum(vec![1, 2]));
    }
    #[test]
    pub fn test_get_sum_2() {
        assert_eq!(31, Solution::get_sum(vec![1, 4, 2, 3]));
    }
}
