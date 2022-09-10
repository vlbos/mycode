// # [2291. Maximum Profit From Trading Stocks](https://leetcode.com/problems/maximum-profit-from-trading-stocks)

// ## Description

// You are given two 0-indexed integer arrays of the same length present and future where present[i] is the current price of the ith stock and future[i] is the price of the ith stock a year in the future.
//  You may buy each stock at most once. You are also given an integer budget representing the amount of money you currently have.

// Return the maximum amount of profit you can make.

// Example 1:

//
// Input: present = [5,4,6,2,3], future = [8,5,4,3,5], budget = 10
// Output: 6
// Explanation: One possible way to maximize your profit is to:
// Buy the 0th, 3rd, and 4th stocks for a total of 5 + 2 + 3 = 10.
// Next year, sell all three stocks for a total of 8 + 3 + 5 = 16.
// The profit you made is 16 - 10 = 6.
// It can be shown that the maximum profit you can make is 6.
//

// Example 2:

//
// Input: present = [2,2,5], future = [3,4,10], budget = 6
// Output: 5
// Explanation: The only possible way to maximize your profit is to:
// Buy the 2nd stock, and make a profit of 10 - 5 = 5.
// It can be shown that the maximum profit you can make is 5.
//

// Example 3:

//
// Input: present = [3,3,12], future = [0,3,15], budget = 10
// Output: 0
// Explanation: One possible way to maximize your profit is to:
// Buy the 1st stock, and make a profit of 3 - 3 = 0.
// It can be shown that the maximum profit you can make is 0.
//

// Constraints:

//
// 	n == present.length == future.length
// 	1 <= n <= 1000
// 	0 <= present[i], future[i] <= 100
// 	0 <= budget <= 1000
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def maximum_profit(self, present: List[int], future: List[int], budget: int) -> int:

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn maximum_profit(present: Vec<i32>, future: Vec<i32>, budget: i32) -> i32 {
        let b = budget as usize;
        let mut dp = vec![0; b + 1];
        for (i, (&p, &f)) in present.iter().zip(&future).enumerate() {
            for j in (p as usize..=b).rev() {
                dp[j] = dp[j].max(f - p + dp[j - p as usize]);
            }
        }
        dp[b]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_profit_1() {
        assert_eq!(
            6,
            Solution::maximum_profit(vec![5, 4, 6, 2, 3], vec![8, 5, 4, 3, 5], 10)
        );
    }
    #[test]
    pub fn test_maximum_profit_2() {
        assert_eq!(
            5,
            Solution::maximum_profit(vec![2, 2, 5], vec![3, 4, 10], 6)
        );
    }
    #[test]
    pub fn test_maximum_profit_3() {
        assert_eq!(
            0,
            Solution::maximum_profit(vec![3, 3, 12], vec![0, 3, 15], 10)
        );
    }
}
