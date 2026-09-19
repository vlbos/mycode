// [3359\. Find Sorted Submatrices With Maximum Element at Most K 🔒](https://leetcode.com/problems/find-sorted-submatrices-with-maximum-element-at-most-k)
// ========================================================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// You are given a 2D matrix `grid` of size `m x n`. You are also given a **non-negative** integer `k`.

// Return the number of **submatrices** of `grid` that satisfy the following conditions:

// *   The maximum element in the submatrix **less than or equal to** `k`.
// *   Each row in the submatrix is sorted in **non-increasing** order.

// A submatrix `(x1, y1, x2, y2)` is a matrix that forms by choosing all cells `grid[x][y]` where `x1 <= x <= x2` and `y1 <= y <= y2`.

// **Example 1:**

// **Input:** grid = \[\[4,3,2,1\],\[8,7,6,1\]\], k = 3

// **Output:** 8

// **Explanation:**

// **[![](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3359.Find%20Sorted%20Submatrices%20With%20Maximum%20Element%20at%20Most%20K/images/mine.png)](https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/3300-3399/3359.Find%20Sorted%20Submatrices%20With%20Maximum%20Element%20at%20Most%20K/images/mine.png)**

// The 8 submatrices are:

// *   `[[1]]`
// *   `[[1]]`
// *   `[[2,1]]`
// *   `[[3,2,1]]`
// *   `[[1],[1]]`
// *   `[[2]]`
// *   `[[3]]`
// *   `[[3,2]]`

// **Example 2:**

// **Input:** grid = \[\[1,1,1\],\[1,1,1\],\[1,1,1\]\], k = 1

// **Output:** 36

// **Explanation:**

// There are 36 submatrices of grid. All submatrices have their maximum element equal to 1.

// **Example 3:**

// **Input:** grid = \[\[1\]\], k = 1

// **Output:** 1

// **Constraints:**

// *   `1 <= m == grid.length <= 103`
// *   `1 <= n == grid[i].length <= 103`
// *   `1 <= grid[i][j] <= 109`
// *   `1 <= k <= 109`

//  long long count_submatrices(vector<vector<int>>& grid, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut submatrices = 0;
        let (m, n) = (grid.len(), grid[0].len());
        let mut widths = vec![0; m];
        for j in 0..n {
            let mut curr_submatrices = 0;
            let mut stack: Vec<Vec<i64>> = vec![];
            for i in 0..m {
                if grid[i][j] > k {
                    widths[i] = 0;
                } else if j > 0 && grid[i][j] > grid[i][j - 1] {
                    widths[i] = 1;
                } else {
                    widths[i] += 1;
                }
                let width = widths[i];
                let mut height = 1;
                while stack.last().is_some_and(|v| v[0] >= width) {
                    let prev = stack.pop().unwrap();
                    curr_submatrices -= (prev[0] - width) * prev[1];
                    height += prev[1];
                }
                curr_submatrices += width;
                submatrices += curr_submatrices;
                stack.push(vec![width, height]);
            }
        }
        submatrices
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_count_submatrices_1() {
        assert_eq!(
            8,
            Solution::count_submatrices(lc_matrix![[4, 3, 2, 1], [8, 7, 6, 1]], 3),
        );
    }
    #[test]
    pub fn test_count_submatrices_2() {
        assert_eq!(
            36,
            Solution::count_submatrices(lc_matrix![[1, 1, 1], [1, 1, 1], [1, 1, 1]], 1)
        );
    }

    pub fn test_count_submatrices_3() {
        assert_eq!(1, Solution::count_submatrices(lc_matrix![[1]], 1));
    }
}
