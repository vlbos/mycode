// ## [3565\. Sequential Grid Path Cover 🔒](https://leetcode.com/problems/sequential-grid-path-cover)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given a 2D array `grid` of size `m x n`, and an integer `k`. There are `k` cells in `grid` containing the values from 1 to `k` **exactly once**, and the rest of the cells have a value 0.

// You can start at any cell, and move from a cell to its neighbors (up, down, left, or right). You must find a path in `grid` which:

// +   Visits each cell in `grid` **exactly once**.
// +   Visits the cells with values from 1 to `k` **in order**.

// Return a 2D array `result` of size `(m * n) x 2`, where `result[i] = [xi, yi]` represents the `ith` cell visited in the path. If there are multiple such paths, you may return **any** one.

// If no such path exists, return an **empty** array.

// **Example 1:**

// **Input:** grid = \[\[0,0,0\],\[0,1,2\]\], k = 2

// **Output:** \[\[0,0\],\[1,0\],\[1,1\],\[1,2\],\[0,2\],\[0,1\]\]

// **Explanation:**

// [![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3500-3599/3565.Sequential%20Grid%20Path%20Cover/images/ezgifcom-animated-gif-maker1.gif)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3500-3599/3565.Sequential%20Grid%20Path%20Cover/images/ezgifcom-animated-gif-maker1.gif)

// **Example 2:**

// **Input:** grid = \[\[1,0,4\],\[3,0,2\]\], k = 4

// **Output:** \[\]

// **Explanation:**

// There is no possible path that satisfies the conditions.

// **Constraints:**

// +   `1 <= m == grid.length <= 5`
// +   `1 <= n == grid[i].length <= 5`
// +   `1 <= k <= m * n`
// +   `0 <= grid[i][j] <= k`
// +   `grid` contains all integers between 1 and `k` **exactly** once.

// vector<vector<int>> find_path(vector<vector<int>>& grid, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_path(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_find_path_1() {
        assert_eq!(
            lc_matrix![[0, 0], [1, 0], [1, 1], [1, 2], [0, 2], [0, 1]],
            Solution::find_path(lc_matrix![[0, 0, 0], [0, 1, 2]], 2)
        );
    }
    #[test]
    pub fn test_find_path_2() {
        assert_eq!(
            vec![],
            Solution::find_path(lc_matrix![[1, 0, 4], [3, 0, 2]], 2)
        );
    }
}
