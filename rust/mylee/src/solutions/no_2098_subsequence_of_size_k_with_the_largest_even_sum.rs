// # [2098. Subsequence of Size K With the Largest Even Sum](https://leetcode.com/problems/subsequence-of-size-k-with-the-largest-even-sum)

// ## Description

// You are given an integer array nums and an integer k. Find the largest even sum of any subsequence of nums that has a length of k.

// Return this sum, or -1 if such a sum does not exist.

// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

// Example 1:

//
// Input: nums = [4,1,5,3,1], k = 3
// Output: 12
// Explanation:
// The subsequence with the largest possible even sum is [4,5,3]. It has a sum of 4 + 5 + 3 = 12.
//

// Example 2:

//
// Input: nums = [4,6,2], k = 3
// Output: 12
// Explanation:
// The subsequence with the largest possible even sum is [4,6,2]. It has a sum of 4 + 6 + 2 = 12.
//

// Example 3:

//
// Input: nums = [1,3,5], k = 1
// Output: -1
// Explanation:
// No subsequence of nums with length 1 has an even sum.
//

// Constraints:

//
// 	1 <= nums.length <= 105
// 	0 <= nums[i] <= 105
// 	1 <= k <= nums.length
//
//  long long largest_even_sum(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn largest_even_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        nums.sort();
        let (n, k) = (nums.len(), k as usize);
        let sum = nums[n - k..].iter().sum::<i64>();
        if sum % 2 == 0 {
            return sum;
        }
        let (min_odd, min_even) = (
            *nums[n - k..].iter().filter(|&x| x % 2 > 0).min().unwrap_or(&-1),
            *nums[n - k..].iter().filter(|&x| x % 2 == 0).min().unwrap_or(&-1),
        );
        let (max_odd, max_even) = (
            *nums[..n-k].iter().filter(|&x| x % 2 > 0).max().unwrap_or(&-1),
            *nums[..n-k].iter().filter(|&x| x % 2 == 0).max().unwrap_or(&-1),
        );
        let mut ans = -1;
        if max_even >= 0 && min_odd >= 0 {
            ans = ans.max(sum + max_even - min_odd);
        }
        if max_odd >= 0 && min_even >= 0 {
            ans = ans.max(sum + max_odd - min_even);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_largest_even_sum_1() {
        assert_eq!(12, Solution::largest_even_sum(vec![4, 1, 5, 3, 1], 3));
    }
    #[test]
    pub fn test_largest_even_sum_2() {
        assert_eq!(12, Solution::largest_even_sum(vec![4, 6, 2], 3));
    }
    #[test]
    pub fn test_largest_even_sum_3() {
        assert_eq!(-1, Solution::largest_even_sum(vec![1, 3, 5], 1));
    }
}
