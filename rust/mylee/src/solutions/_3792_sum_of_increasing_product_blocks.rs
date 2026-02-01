// [3792\. Sum of Increasing Product Blocks 🔒](https://leetcode.com/problems/sum-of-increasing-product-blocks)
// ============================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given an integer `n`.

// A sequence is formed as follows:

// *   The `1st` block contains `1`.
// *   The `2nd` block contains `2 * 3`.
// *   The `ith` block is the product of the next `i` consecutive integers.

// Let `F(n)` be the sum of the first `n` blocks.

// Return an integer denoting `F(n)` **modulo** `109 + 7`.

// **Example 1:**

// **Input:** n = 3

// **Output:** 127

// **Explanation:**​​​​​​​

// *   Block 1: `1`
// *   Block 2: `2 * 3 = 6`
// *   Block 3: `4 * 5 * 6 = 120`

// `F(3) = 1 + 6 + 120 = 127`

// **Example 2:**

// **Input:** n = 7

// **Output:** 6997165

// **Explanation:**

// *   Block 1: `1`
// *   Block 2: `2 * 3 = 6`
// *   Block 3: `4 * 5 * 6 = 120`
// *   Block 4: `7 * 8 * 9 * 10 = 5040`
// *   Block 5: `11 * 12 * 13 * 14 * 15 = 360360`
// *   Block 6: `16 * 17 * 18 * 19 * 20 * 21 = 39070080`
// *   Block 7: `22 * 23 * 24 * 25 * 26 * 27 * 28 = 5967561600`

// `F(7) = 6006997207 % (109 + 7) = 6997165`

// **Constraints:**

// *   `1 <= n <= 1000`

// int sum_of_blocks(int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn sum_of_blocks(n: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sum_of_blocks_1() {
        assert_eq!(127, Solution::sum_of_blocks(3));
    }
    #[test]
    pub fn test_sum_of_blocks_2() {
        assert_eq!(6997165, Solution::sum_of_blocks(7));
    }
}
