// ## [3596\. Minimum Cost Path with Alternating Directions I 🔒](https://leetcode.com/problems/minimum-cost-path-with-alternating-directions-i)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given two integers `m` and `n` representing the number of rows and columns of a grid, respectively.

// The cost to enter cell `(i, j)` is defined as `(i + 1) * (j + 1)`.

// You start at cell `(0, 0)` on move 1.

// At each step, you move to an **adjacent** cell, following an alternating pattern:

// +   On **odd-numbered** moves, you must move either **right** or **down**.
// +   On **even-numbered** moves, you must move either **left** or **up**.

// Return the **minimum** total cost required to reach `(m - 1, n - 1)`. If it is impossible, return -1.

// **Example 1:**

// **Input:** m = 1, n = 1

// **Output:** 1

// **Explanation:**

// +   You start at cell `(0, 0)`.
// +   The cost to enter `(0, 0)` is `(0 + 1) * (0 + 1) = 1`.
// +   Since you're at the destination, the total cost is 1.

// **Example 2:**

// **Input:** m = 2, n = 1

// **Output:** 3

// **Explanation:**

// +   You start at cell `(0, 0)` with cost `(0 + 1) * (0 + 1) = 1`.
// +   Move 1 (odd): You can move down to `(1, 0)` with cost `(1 + 1) * (0 + 1) = 2`.
// +   Thus, the total cost is `1 + 2 = 3`.

// **Constraints:**

// +   `1 <= m, n <= 106`

// int min_cost(int m, int n) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_cost(m: i32, n: i32) -> i32 {
        if m == 1 && n == 1 {
            return 1;
        }
        if m + n == 3 {
            return 3;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_cost_1() {
        assert_eq!(1, Solution::min_cost(1, 1));
    }
    #[test]
    pub fn test_min_cost_2() {
        assert_eq!(3, Solution::min_cost(2, 1));
    }
}
