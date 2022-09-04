// # [2123. Minimum Operations to Remove Adjacent Ones in Matrix](https://leetcode.com/problems/minimum-operations-to-remove-adjacent-ones-in-matrix)

// ## Description

// You are given a 0-indexed binary matrix grid. In one operation, you can flip any 1 in grid to be 0.

// A binary matrix is well-isolated if there is no 1 in the matrix that is 4-directionally connected (i.e., horizontal and vertical) to another 1.

// Return the minimum number of operations to make grid well-isolated.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2123.Minimum%20Operations%20to%20Remove%20Adjacent%20Ones%20in%20Matrix/images/image-20211223181501-1.png" style="width: 644px; height: 250px;" />
//
// Input: grid = [[1,1,0],[0,1,1],[1,1,1]]
// Output: 3
// Explanation: Use 3 operations to change grid[0][1], grid[1][2], and grid[2][1] to 0.
// After, no more 1's are 4-directionally connected and grid is well-isolated.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2123.Minimum%20Operations%20to%20Remove%20Adjacent%20Ones%20in%20Matrix/images/image-20211223181518-2.png" style="height: 250px; width: 255px;" />
//
// Input: grid = [[0,0,0],[0,0,0],[0,0,0]]
// Output: 0
// Explanation: There are no 1's in grid and it is well-isolated.
// No operations were done so return 0.
//

// Example 3:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2123.Minimum%20Operations%20to%20Remove%20Adjacent%20Ones%20in%20Matrix/images/image-20211223181817-3.png" style="width: 165px; height: 167px;" />
//
// Input: grid = [[0,1],[1,0]]
// Output: 0
// Explanation: None of the 1's are 4-directionally connected and grid is well-isolated.
// No operations were done so return 0.
//

// Constraints:

//
// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m, n <= 300
// 	grid[i][j] is either 0 or 1.
//
//  int minimum_operations(vector<vector<int>>& grid) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = vec![vec![None; n]; m];
        let mut matches = vec![vec![None; n]; m];
        let mut ans = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 0 || matches[i][j].is_some() {
                    continue;
                }
                seen[i][j] = Some((i, j));
                if dfs((i, j), (i, j), &grid, &mut seen, &mut matches) {
                    ans += 1;
                }
            }
        }
        fn dfs(
            ij: (usize, usize),
            id: (usize, usize),
            grid: &Vec<Vec<i32>>,
            seen: &mut Vec<Vec<Option<(usize, usize)>>>,
            matches: &mut Vec<Vec<Option<(usize, usize)>>>,
        ) -> bool {
            let (m, n) = (grid.len(), grid[0].len());
            let dirs = [0, 1, 0, -1, 0];
            let (i, j) = ij;
            for (&di, &dj) in dirs[1..].iter().zip(&dirs[..dirs.len() - 1]) {
                let (x, y) = (i as i32 + di, j as i32 + dj);
                if x < 0 || x == m as i32 || y < 0 || y == n as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if grid[x][y] == 0 || seen[x][y] == Some(id) {
                    continue;
                }
                seen[x][y] = Some(id);
                if matches[x][y].is_none() || dfs(matches[x][y].unwrap(), id, grid, seen, matches) {
                    matches[x][y] = Some(ij);
                    matches[i][j] = Some((x, y));
                    return true;
                }
            }
            false
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_operations_1() {
        assert_eq!(
            3,
            Solution::minimum_operations(vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 1, 1]])
        );
    }
    #[test]
    pub fn test_minimum_operations_2() {
        assert_eq!(
            0,
            Solution::minimum_operations(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]])
        );
    }
    #[test]
    pub fn test_minimum_operations_3() {
        assert_eq!(
            0,
            Solution::minimum_operations(vec![vec![0, 1], vec![1, 0]])
        );
    }
}
