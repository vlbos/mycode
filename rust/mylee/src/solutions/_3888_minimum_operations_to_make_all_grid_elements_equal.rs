// 3888. Minimum Operations to Make All Grid Elements Equal
// ## Description
// You are given a 2D integer array `grid` of size `m × n`, and an integer `k`.
// In one operation, you can:
// * Select any `k x k` **submatrix** of `grid`, and
// * Increment **all elements** inside that **submatrix** by 1.
// Return the **minimum** number of operations required to make all elements in the grid **equal**. If it is not possible, return -1.
// A submatrix `(x1, y1, x2, y2)` is a matrix that forms by choosing all cells `matrix[x][y]` where `x1 \<= x \<= x2` and `y1 \<= y \<= y2`.

// **Example 1:**
// **Input:** grid = [[3,3,5],[3,3,5]], k = 2
// **Output:** 2
// **Explanation:**
// Choose the left `2 x 2` submatrix (covering the first two columns) and apply the operation twice.
// * After 1 operation: `[[4, 4, 5], [4, 4, 5]]`
// * After 2 operations: `[[5, 5, 5], [5, 5, 5]]`
// All elements become equal to 5. Thus, the minimum number of operations is 2.
// **Example 2:**
// **Input:** grid = [[1,2],[2,3]], k = 1
// **Output:** 4
// **Explanation:**
// Since `k = 1`, each operation increments a single cell `grid[i][j]` by 1. To make all elements equal, the final value must be 3.
// * Increase `grid[0][0] = 1` to 3, requiring 2 operations.
// * Increase `grid[0][1] = 2` to 3, requiring 1 operation.
// * Increase `grid[1][0] = 2` to 3, requiring 1 operation.
// Thus, the minimum number of operations is `2 + 1 + 1 + 0 = 4`.

// **Constraints:**
// * `1 \<= m == grid.length \<= 1000`
// * `1 \<= n == grid[i].length \<= 1000`
// * `-105 \<= grid[i][j] \<= 105`
// * `1 \<= k \<= min(m, n)`

// long long min_operations(vector<vector<int>>& grid, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_operations(mut grid: Vec<Vec<i32>>, k: i32) -> i64 {
        let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
        let mx = grid
            .iter()
            .map(|row| *row.iter().max().unwrap())
            .max()
            .unwrap() as i64;
        let check = |target: i64| {
            let mut diff = vec![vec![0; n + 2]; m + 2];
            let mut total_ops = 0;
            for (i, row) in grid.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    diff[i + 1][j + 1] += diff[i][j + 1] + diff[i + 1][j] - diff[i][j];
                    let cur_val = val as i64 + diff[i + 1][j + 1];
                    if cur_val > target {
                        return -1;
                    }
                    if cur_val < target {
                        if i + k > m || j + k > n {
                            return -1;
                        }
                        let needed = target - cur_val;
                        total_ops += needed;
                        diff[i + 1][j + 1] += needed;
                        diff[i + 1 + k][j + 1] -= needed;
                        diff[i + 1][j + 1 + k] -= needed;
                        diff[i + 1 + k][j + 1 + k] += needed;
                    }
                }
            }
            total_ops
        };
        let ans = check(mx);
        if ans != -1 {
            return ans;
        }
        check(mx + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(
            2,
            Solution::min_operations(lc_matrix![[3, 3, 5], [3, 3, 5]], 2),
        );
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(4, Solution::min_operations(lc_matrix![[1, 2], [2, 3]], 1));
    }
}
