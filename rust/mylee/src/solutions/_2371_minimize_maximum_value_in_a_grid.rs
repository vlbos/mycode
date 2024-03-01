// # [2371. Minimize Maximum Value in a Grid](https://leetcode.com/problems/minimize-maximum-value-in-a-grid)

// ## Description

// You are given an m x n integer matrix grid containing distinct positive integers.

// You have to replace each integer in the matrix with a positive integer satisfying the following conditions:

// 	The relative order of every two elements that are in the same row or column should stay the same after the replacements.
// 	The maximum number in the matrix after the replacements should be as small as possible.

// The relative order stays the same if for all pairs of elements in the original matrix such that grid[r1][c1] > grid[r2][c2] where either r1 == r2 or c1 == c2,
// then it must be true that grid[r1][c1] > grid[r2][c2] after the replacements.

// For example, if grid = [[2, 4, 5], [7, 3, 9]] then a good replacement could be either grid = [[1, 2, 3], [2, 1, 4]] or grid = [[1, 2, 3], [3, 1, 4]].

// Return the resulting matrix. If there are multiple answers, return any of them.

// Example 1:
// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2300-2399/2371.Minimize%20Maximum%20Value%20in%20a%20Grid/images/grid2drawio.png" style="width: 371px; height: 121px;" />
//
// Input: grid = [[3,1],[2,5]]
// Output: [[2,1],[1,2]]
// Explanation: The above diagram shows a valid replacement.
// The maximum number in the matrix is 2. It can be shown that no smaller value can be obtained.
//

// Example 2:

//
// Input: grid = [[10]]
// Output: [[1]]
// Explanation: We replace the only number in the matrix with 1.
//

// Constraints:

//
// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m, n <= 1000
// 	1 <= m * n <= 10^5
// 	1 <= grid[i][j] <= 10^9
// 	grid consists of distinct integers.
//

// vector<vector<int>> min_score(vector<vector<int>>& grid) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_score(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![vec![0; n]; m];
        let mut arr = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                arr.push((v, i, j));
            }
        }
        arr.sort();
        let (mut rows, mut cols) = (vec![0; m], vec![0; n]);
        for &(_, i, j) in &arr {
            let next = rows[i].max(cols[j]) + 1;
            ans[i][j] = next;
            rows[i] = next;
            cols[j] = next;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_score_1() {
        assert_eq!(
            vec![vec![2, 1], vec![1, 2]],
            Solution::min_score(vec![vec![3, 1], vec![2, 5]])
        );
    }
    #[test]
    pub fn test_min_score_2() {
        assert_eq!(vec![vec![1]], Solution::min_score(vec![vec![10]]));
    }
}
