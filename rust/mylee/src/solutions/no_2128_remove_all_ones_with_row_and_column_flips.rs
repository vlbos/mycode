// [2128\. Remove All Ones With Row and Column Flips (Medium)](https://leetcode.com/problems/remove-all-ones-with-row-and-column-flips/)[](https://leetcode.ca/2022-01-20-2128-Remove-All-Ones-With-Row-and-Column-Flips/#2128-remove-all-ones-with-row-and-column-flips-medium)
// =============================================================================================================================================================================================================================================================================

// You are given an `m x n` binary matrix `grid`.

// In one operation, you can choose **any** row or column and flip each value in that row or column (i.e., changing all `0`'s to `1`'s, and all `1`'s to `0`'s).

// Return `true` _if it is possible to remove all_ `1`_'s from_ `grid` using **any** number of operations or `false` otherwise.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2022/01/03/image-20220103191300-1.png)

// **Input:** grid = \[\[0,1,0\],\[1,0,1\],\[0,1,0\]\]
// **Output:** true
// **Explanation:** One possible way to remove all 1's from grid is to:
// - Flip the middle row
// - Flip the middle column

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2022/01/03/image-20220103181204-7.png)

// **Input:** grid = \[\[1,1,0\],\[0,0,0\],\[0,0,0\]\]
// **Output:** false
// **Explanation:** It is impossible to remove all 1's from grid.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2022/01/03/image-20220103181224-8.png)

// **Input:** grid = \[\[0\]\]
// **Output:** true
// **Explanation:** There are no 1's in grid.

// **Constraints:**

// *   `m == grid.length`
// *   `n == grid[i].length`
// *   `1 <= m, n <= 300`
// *   `grid[i][j]` is either `0` or `1`.

// **Companies**:
// [Google](https://leetcode.com/company/google)

// **Related Topics**:
// [Array](https://leetcode.com/tag/array/), [Math](https://leetcode.com/tag/math/), [Bit Manipulation](https://leetcode.com/tag/bit-manipulation/), [Matrix](https://leetcode.com/tag/matrix/)

// **Similar Questions**:

// *   [Score After Flipping Matrix (Medium)](https://leetcode.com/problems/score-after-flipping-matrix/)
// *   [Minimum Number of Flips to Convert Binary Matrix to Zero Matrix (Hard)](https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix/)
// *   [Minimum Operations to Remove Adjacent Ones in Matrix (Hard)](https://leetcode.com/problems/minimum-operations-to-remove-adjacent-ones-in-matrix/)

// Solution 1.[](https://leetcode.ca/2022-01-20-2128-Remove-All-Ones-With-Row-and-Column-Flips/#solution-1)
// --------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/remove-all-ones-with-row-and-column-flips/
//     // Time: O(MN)
//     // Space: O(1)
//     class Solution {
//     public:
//         bool remove_ones(vector<vector<int>>& A) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn remove_ones(matrix: Vec<Vec<i32>>) -> bool {
        let mut matrix = matrix;
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            if matrix[i][0] > 0 {
                for j in 0..n {
                    matrix[i][j] = 1 - matrix[i][j];
                }
            }
            if i > 0 {
                for j in 0..n {
                    if matrix[i][j] != matrix[0][j] {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_remove_ones_1() {
        assert!(Solution::remove_ones(vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0]
        ]));
    }
    #[test]
    pub fn test_remove_ones_2() {
        assert!(!Solution::remove_ones(vec![
            vec![1, 1, 0],
            vec![0, 0, 0],
            vec![0, 0, 0]
        ]));
    }
    #[test]
    pub fn test_remove_ones_3() {
        assert!(Solution::remove_ones(vec![vec![0]]));
    }
}
