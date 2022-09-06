// # [2174. Remove All Ones With Row and Column Flips II](https://leetcode.com/problems/remove-all-ones-with-row-and-column-flips-ii)

// ## Description

// You are given a 0-indexed m x n binary matrix grid.

// In one operation, you can choose any i and j that meet the following conditions:

//
// 	0 <= i < m
// 	0 <= j < n
// 	grid[i][j] == 1
//

// and change the values of all cells in row i and column j to zero.

// Return the minimum number of operations needed to remove all 1's from grid.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2174.Remove%20All%20Ones%20With%20Row%20and%20Column%20Flips%20II/images/image-20220213162716-1.png" style="width: 709px; height: 200px;" />
//
// Input: grid = [[1,1,1],[1,1,1],[0,1,0]]
// Output: 2
// Explanation:
// In the first operation, change all cell values of row 1 and column 1 to zero.
// In the second operation, change all cell values of row 0 and column 0 to zero.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2174.Remove%20All%20Ones%20With%20Row%20and%20Column%20Flips%20II/images/image-20220213162737-2.png" style="width: 734px; height: 200px;" />
//
// Input: grid = [[0,1,0],[1,0,1],[0,1,0]]
// Output: 2
// Explanation:
// In the first operation, change all cell values of row 1 and column 0 to zero.
// In the second operation, change all cell values of row 2 and column 1 to zero.
// Note that we cannot perform an operation using row 1 and column 1 because grid[1][1] != 1.
//

// Example 3:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2100-2199/2174.Remove%20All%20Ones%20With%20Row%20and%20Column%20Flips%20II/images/image-20220213162752-3.png" style="width: 156px; height: 150px;" />
//
// Input: grid = [[0,0],[0,0]]
// Output: 0
// Explanation:
// There are no 1's to remove so return 0.
//

// Constraints:

//
// 	m == grid.length
// 	n == grid[i].length
// 	1 <= m, n <= 15
// 	1 <= m * n <= 15
// 	grid[i][j] is either 0 or 1.
//

// int remove_ones(vector<vector<int>>& grid) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn remove_ones(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut one: HashSet<(usize, usize)> =
            grid.iter()
                .enumerate()
                .fold(HashSet::new(), |mut r, (i, row)| {
                    r = r
                        .union(
                            &row.iter()
                                .enumerate()
                                .fold(HashSet::new(), |mut c, (j, &v)| {
                                    if v == 1 {
                                        c.insert((i, j));
                                    };
                                    c
                                }),
                        )
                        .cloned()
                        .collect();
                    r
                });
        let mut ans = i32::MAX;
        fn dfs(res: i32, ans: &mut i32, grid: &Vec<Vec<i32>>, one: &mut HashSet<(usize, usize)>) {
            if one.is_empty() {
                *ans = res.min(*ans);
                return;
            }
            for (curr_r, curr_c) in one.clone() {
                let mut deleted = HashSet::new();
                for c in 0..grid[0].len() {
                    if one.contains(&(curr_r, c)) {
                        one.remove(&(curr_r, c));
                        deleted.insert((curr_r, c));
                    }
                }
                for r in 0..grid.len() {
                    if one.contains(&(r, curr_c)) {
                        one.remove(&(r, curr_c));
                        deleted.insert((r, curr_c));
                    }
                }
                dfs(res + 1, ans, grid, one);
                *one = one.union(&deleted).cloned().collect();
            }
        }
        dfs(0, &mut ans, &grid, &mut one);
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_remove_ones_1() {
        assert_eq!(
            2,
            Solution::remove_ones(vec![vec![1, 1, 1], vec![1, 1, 1], vec![0, 1, 0]])
        );
    }
    #[test]
    pub fn test_remove_ones_2() {
        assert_eq!(
            2,
            Solution::remove_ones(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]])
        );
    }
    #[test]
    pub fn test_remove_ones_3() {
        assert_eq!(0, Solution::remove_ones(vec![vec![0, 0], vec![0, 0]]));
    }
}
