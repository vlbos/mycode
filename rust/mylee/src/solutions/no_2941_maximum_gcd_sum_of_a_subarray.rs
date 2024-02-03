// # [2941. Maximum GCD-Sum of a Subarray](https://leetcode.com/problems/maximum-gcd-sum-of-a-subarray)

// ## Description

// You are given an array of integers nums and an integer k.

// The gcd-sum of an array a is calculated as follows:

// 	Let s be the sum of all the elements of a.
// 	Let g be the greatest common divisor of all the elements of a.
// 	The gcd-sum of a is equal to s * g.

// Return the maximum gcd-sum of a subarray of nums with at least k elements.

//
// Example 1:

// Input: nums = [2,1,4,4,4,2], k = 2
// Output: 48
// Explanation: We take the subarray [4,4,4], the gcd-sum of this array is 4 * (4 + 4 + 4) = 48.
// It can be shown that we can not select any other subarray with a gcd-sum greater than 48.

// Example 2:

// Input: nums = [7,3,9,4], k = 1
// Output: 81
// Explanation: We take the subarray [9], the gcd-sum of this array is 9 * 9 = 81.
// It can be shown that we can not select any other subarray with a gcd-sum greater than 81.

//
// Constraints:

// 	n == nums.length
// 	1  <= n  <= 105
// 	1  <= nums[i]  <= 106
// 	1  <= k  <= n

//     long long max_gcd_sum(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_gcd_sum(nums: Vec<i32>, k: i32) -> i64 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_gcd_sum_1() {
        assert_eq!(48, Solution::max_gcd_sum(vec![2, 1, 4, 4, 4, 2], 2));
    }
    #[test]
    pub fn test_max_gcd_sum_2() {
        assert_eq!(81, Solution::max_gcd_sum(vec![7, 3, 9, 4], 1));
    }
}
