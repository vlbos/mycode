/*
 * @lc app=leetcode id=840 lang=rust
 *
 * [840] Magic Squares In Grid
 */

// @lc code=start
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m < 3 || n < 3 {
            return 0;
        }
        let magic = |grid: &Vec<Vec<i32>>, r: usize, c: usize| -> bool {
            let mut count = vec![0; 16];
            for i in 0..3 {
                for j in 0..3 {
                    count[grid[r + i][c + j] as usize] += 1;
                }
            }
            if count[1..10].iter().any(|x| *x != 1) {
                return false;
            }
            let mut cols = vec![0; 3];
            for i in 0..3 {
                let mut rowsum = 0;
                for j in 0..3 {
                    cols[j] += grid[r + i][c + j];
                    rowsum += grid[r + i][c + j];
                }
                if rowsum != 15 {
                    return false;
                }
            }
            if cols.iter().any(|x| *x != 15) {
                return false;
            }
            true
        };
        let mut ans = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                if grid[i + 1][j + 1] != 5 {
                    continue;
                }
                if magic(&grid, i, j) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
