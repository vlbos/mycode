// [3990. Create Grid With Exactly K Paths II](https://leetcode.com/problems/create-grid-with-exactly-k-paths-ii/)

// You are given an integer `k`.
// Construct **any** grid consisting only of the characters `'.'` and `'#'`, where:
// * `'.'` represents a free cell.
// * `'#'` represents an obstacle cell.
// The grid must contain **at most** 25 rows and **at most** 25 columns.
// A **valid path** is a sequence of free cells that:
// * Starts at the top-left cell `(0, 0)`.
// * Ends at the bottom-right cell `(m - 1, n - 1)`, where `m` and `n` are the dimensions of your constructed grid.
// * Moves only:
// * Right, from `(i, j)` to `(i, j + 1)`, or
// * Down, from `(i, j)` to `(i + 1, j)`.
// Return any grid such that there are **exactly `k` valid paths** from the top-left cell to the bottom-right cell. If no such grid exists, return an empty array.
// **Example 1:**
// **Input:** k = 2
// **Output:** ["..#","#..","#.."]
// **Explanation:**
// ![](./Create%20Grid%20With%20Exactly%20K%20Paths%20II%20-%20LeetCode_files/screenshot-2026-05-31-at-82224pm.png)
// The grid contains exactly 2 valid paths from `(0, 0)` to `(2, 2)`:
// * `(0, 0) → (0, 1) → (1, 1) → (1, 2) → (2, 2)`
// * `(0, 0) → (0, 1) → (1, 1) → (2, 1) → (2, 2)`
// **Example 2:**
// **Input:** k = 3
// **Output:** ["...","#..","#.."]
// **Explanation:**
// **​​​​​​​**![](./Create%20Grid%20With%20Exactly%20K%20Paths%20II%20-%20LeetCode_files/screenshot-2026-05-31-at-82251pm.png)
// The grid contains exactly 3 valid paths from `(0, 0)` to `(2, 2)`:
// * `(0, 0) → (0, 1) → (0, 2) → (1, 2) → (2, 2)`
// * `(0, 0) → (0, 1) → (1, 1) → (1, 2) → (2, 2)`
// * `(0, 0) → (0, 1) → (1, 1) → (2, 1) → (2, 2)`
// **Constraints:**​​​​​​​
// * `1 \<= k \<= 1000`

impl Solution {
    pub fn create_grid(k: i32) -> Vec<String> {
        let w = 32 - k.leading_zeros() as usize;
        let (m, n) = (w * 2, w + 3);
        let mut b = vec!['#'; n];
        b[n - 1] = '.';
        let mut a = vec![b; m];
        for j in 0..w {
            let i = j * 2;
            a[i][j..j + 2].fill('.');
            a[i + 1][j..j + 2].fill('.');
        }
        for i in 0..w {
            if k >> i & 1 == 1 {
                for j in i + 2..n - 1 {
                    a[i * 2][j] = '.';
                }
            }
        }
        a.into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
use crate::lc_vec_s;
    // use crate::solutions::util::test_tools::{assert_equivalent, map_to_string};
    #[test]
    pub fn test_create_grid_1() {
        let tar = Solution::create_grid(2);
        let src = lc_vec_s!["..#","#..","#.."];
        assert_eq!(&tar, &src);
    }
 #[test]
    pub fn test_create_grid_2() {
        let tar = Solution::create_grid(3);
        let src = lc_vec_s!["...","#..","#.."];
        assert_eq!(&tar, &src);
    }
}

