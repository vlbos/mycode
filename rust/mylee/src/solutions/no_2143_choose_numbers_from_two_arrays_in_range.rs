// # [2143. Choose Numbers From Two Arrays in Range](https://leetcode.com/problems/choose-numbers-from-two-arrays-in-range)

// ## Description

// You are given two 0-indexed integer arrays nums1 and nums2 of length n.

// A range [l, r] (inclusive) where 0 <= l <= r < n is balanced if:

//
// 	For every i in the range [l, r], you pick either nums1[i] or nums2[i].
// 	The sum of the numbers you pick from nums1 equals to the sum of the numbers you pick from nums2 (the sum is considered to be 0 if you pick no numbers from an array).
//

// Two balanced ranges from [l1, r1] and [l2, r2] are considered to be different if at least one of the following is true:

//
// 	l1 != l2
// 	r1 != r2
// 	nums1[i] is picked in the first range, and nums2[i] is picked in the second range or vice versa for at least one i.
//

// Return the number of different ranges that are balanced. Since the answer may be very large, return it modulo 109 + 7.

// Example 1:

//
// Input: nums1 = [1,2,5], nums2 = [2,6,3]
// Output: 3
// Explanation: The balanced ranges are:
// - [0, 1] where we choose nums2[0], and nums1[1].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 2 = 2.
// - [0, 2] where we choose nums1[0], nums2[1], and nums1[2].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 1 + 5 = 6.
// - [0, 2] where we choose nums1[0], nums1[1], and nums2[2].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 1 + 2 = 3.
// Note that the second and third balanced ranges are different.
// In the second balanced range, we choose nums2[1] and in the third balanced range, we choose nums1[1].
//

// Example 2:

//
// Input: nums1 = [0,1], nums2 = [1,0]
// Output: 4
// Explanation: The balanced ranges are:
// - [0, 0] where we choose nums1[0].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 0 = 0.
// - [1, 1] where we choose nums2[1].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 0 = 0.
// - [0, 1] where we choose nums1[0] and nums2[1].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 0 = 0.
// - [0, 1] where we choose nums2[0] and nums1[1].
//   The sum of the numbers chosen from nums1 equals the sum of the numbers chosen from nums2: 1 = 1.
//

// Constraints:

//
// 	n == nums1.length == nums2.length
// 	1 <= n <= 100
// 	0 <= nums1[i], nums2[i] <= 100
//

//  int count_sub_ranges(vector<int>& nums1, vector<int>& nums2) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_sub_ranges(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let p = 1_000_000_007;
        let mut ans = 0;
        let mut dp = HashMap::new();
        for (&n1, &n2) in nums1.iter().zip(&nums2) {
            let mut new_dp = HashMap::new();
            *new_dp.entry(n1).or_insert(0) += 1;
            *new_dp.entry(-n2).or_insert(0) += 1;
            for (&v, &c) in &dp {
                new_dp
                    .entry((v + n1))
                    .and_modify(|v| *v = (*v + c) % p)
                    .or_insert(c % p);
                new_dp
                    .entry((v - n2))
                    .and_modify(|v| *v = (*v + c) % p)
                    .or_insert(c % p);
            }
            dp = new_dp;
            if let Some(&v) = dp.get(&0) {
                ans = (ans + v) % p;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_sub_ranges_1() {
        assert_eq!(3, Solution::count_sub_ranges(vec![1, 2, 5], vec![2, 6, 3]));
    }
    #[test]
    pub fn test_count_sub_ranges_2() {
        assert_eq!(4, Solution::count_sub_ranges(vec![0, 1], vec![1, 0]));
    }
}
