// # [3183. The Number of Ways to Make the Sum 🔒](https://leetcode.com/problems/the-number-of-ways-to-make-the-sum)

// ## Description

//

// You have an infinite number of coins with values 1, 2, and 6, and only 2 coins with value 4.
// Given an integer n, return the number of ways to make the sum of n with the coins you have.
// Since the answer may be very large, return it modulo 109 + 7.
// Note that the order of the coins doesn't matter and [2, 2, 3] is the same as [2, 3, 2].

//
// Example 1:

//
// Input: n = 4

// Output: 4

// Explanation:

// Here are the four combinations: [1, 1, 1, 1], [1, 1, 2], [2, 2], [4].
//

// Example 2:

//
// Input: n = 12

// Output: 22

// Explanation:

// Note that [4, 4, 4] is not a valid combination since we cannot use 4 three times.
//

// Example 3:

//
// Input: n = 5

// Output: 4

// Explanation:

// Here are the four combinations: [1, 1, 1, 1, 1], [1, 1, 1, 2], [1, 2, 2], [1, 4].
//

//
// Constraints:

//
// 	1 <= n <= 105
//

//     int number_of_ways(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_of_ways(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 1];
        f[0] = 1;
        for x in [1, 2, 6] {
            for y in x..=n {
                f[y] = (f[y] + f[y - x]) % 1_000_000_007;
            }
        }
        let mut ans = f[n];
        if n >= 4 {
            ans = (ans + f[n - 4]) % 1_000_000_007;
        }
        if n >= 8 {
            ans = (ans + f[n - 8]) % 1_000_000_007;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_number_of_ways_1() {
        assert_eq!(4, Solution::number_of_ways(4));
    }
    #[test]
    pub fn test_number_of_ways_2() {
        assert_eq!(22, Solution::number_of_ways(12));
    }
    #[test]
    pub fn test_number_of_ways_3() {
        assert_eq!(4, Solution::number_of_ways(5));
    }
}
