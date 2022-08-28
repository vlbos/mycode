// # [2291. Maximum Profit From Trading Stocks](https://leetcode.com/problems/maximum-profit-from-trading-stocks)

// [中文文档](/solution/2200-2299/2291.Maximum%20Profit%20From%20Trading%20Stocks/README.md)

// ## Description

// <p>You are given two <strong>0-indexed</strong> integer arrays of the same length <code>present</code> and <code>future</code> where <code>present[i]</code> is the current price of the <code>i<sup>th</sup></code> stock and <code>future[i]</code> is the price of the <code>i<sup>th</sup></code> stock a year in the future. You may buy each stock at most <strong>once</strong>. You are also given an integer <code>budget</code> representing the amount of money you currently have.</p>

// <p>Return <em>the maximum amount of profit you can make.</em></p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> present = [5,4,6,2,3], future = [8,5,4,3,5], budget = 10
// <strong>Output:</strong> 6
// <strong>Explanation:</strong> One possible way to maximize your profit is to:
// Buy the 0<sup>th</sup>, 3<sup>rd</sup>, and 4<sup>th</sup> stocks for a total of 5 + 2 + 3 = 10.
// Next year, sell all three stocks for a total of 8 + 3 + 5 = 16.
// The profit you made is 16 - 10 = 6.
// It can be shown that the maximum profit you can make is 6.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> present = [2,2,5], future = [3,4,10], budget = 6
// <strong>Output:</strong> 5
// <strong>Explanation:</strong> The only possible way to maximize your profit is to:
// Buy the 2<sup>nd</sup> stock, and make a profit of 10 - 5 = 5.
// It can be shown that the maximum profit you can make is 5.
// </pre>

// <p><strong>Example 3:</strong></p>

// <pre>
// <strong>Input:</strong> present = [3,3,12], future = [0,3,15], budget = 10
// <strong>Output:</strong> 0
// <strong>Explanation:</strong> One possible way to maximize your profit is to:
// Buy the 1<sup>st</sup> stock, and make a profit of 3 - 3 = 0.
// It can be shown that the maximum profit you can make is 0.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>n == present.length == future.length</code></li>
// 	<li><code>1 &lt;= n &lt;= 1000</code></li>
// 	<li><code>0 &lt;= present[i], future[i] &lt;= 100</code></li>
// 	<li><code>0 &lt;= budget &lt;= 1000</code></li>
// </ul>

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def maximumProfit(self, present: List[int], future: List[int], budget: int) -> int:

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
