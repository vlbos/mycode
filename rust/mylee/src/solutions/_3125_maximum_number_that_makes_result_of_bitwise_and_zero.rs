// # [3125. Maximum Number That Makes Result of Bitwise AND Zero 🔒](https://leetcode.com/problems/maximum-number-that-makes-result-of-bitwise-and-zero)

// ## Description

//

// Given an integer n, return the maximum integer x such that x <= n,
// and the bitwise AND of all the numbers in the range [x, n] is 0.

//
// Example 1:

//
// Input: n = 7

// Output: 3

// Explanation:

// The bitwise AND of [6, 7] is 6.
// The bitwise AND of [5, 6, 7] is 4.
// The bitwise AND of [4, 5, 6, 7] is 4.
// The bitwise AND of [3, 4, 5, 6, 7] is 0.
//

// Example 2:

//
// Input: n = 9

// Output: 7

// Explanation:

// The bitwise AND of [7, 8, 9] is 0.
//

// Example 3:

//
// Input: n = 17

// Output: 15

// Explanation:

// The bitwise AND of [15, 16, 17] is 0.
//

//
// Constraints:

//
// 	1 <= n <= 1015
//

//     long long max_number(long long n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_number(n: i64) -> i64 {
        (1i64 << (63 - n.leading_zeros())) - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_max_number_1() {
        assert_eq!(3, Solution::max_number(7));
    }
    #[test]
    pub fn test_max_number_2() {
        assert_eq!(7, Solution::max_number(9));
    }
    #[test]
    pub fn test_max_number_3() {
        assert_eq!(15, Solution::max_number(17));
    }
}
