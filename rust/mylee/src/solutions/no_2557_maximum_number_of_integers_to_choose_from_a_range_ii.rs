// # [2557. Maximum Number of Integers to Choose From a Range II](https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-ii)

// You are given an integer array banned and two integers n and maxSum. You are choosing some number of integers following the below rules:

// 	The chosen integers have to be in the range [1, n].
// 	Each integer can be chosen at most once.
// 	The chosen integers should not be in the array banned.
// 	The sum of the chosen integers should not exceed maxSum.

// Return the maximum number of integers you can choose following the mentioned rules.

// Example 1:

// Input: banned = [1,4,6], n = 6, maxSum = 4
// Output: 1
// Explanation: You can choose the integer 3.
// 3 is in the range [1, 6], and do not appear in banned. The sum of the chosen integers is 3, which does not exceed maxSum.

// Example 2:

// Input: banned = [4,3,5,6], n = 7, maxSum = 18
// Output: 3
// Explanation: You can choose the integers 1, 2, and 7.
// All these integers are in the range [1, 7], all do not appear in banned, and their sum is 18, which does not exceed maxSum.

// Constraints:

// 	1 <= banned.length <= 105
// 	1 <= banned[i] <= n <= 109
// 	1 <= maxSum <= 1015
//   int max_count(vector<int>& banned, int n, long long max_sum) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i64) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_count_1() {
        assert_eq!(1, Solution::max_count(vec![1, 4, 6], 6, 4));
    }
    #[test]
    pub fn test_max_count_2() {
        assert_eq!(3, Solution::max_count(vec![4, 3, 5, 6], 7, 18));
    }
}
