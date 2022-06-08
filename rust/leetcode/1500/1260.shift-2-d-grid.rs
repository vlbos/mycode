/*
 * @lc app=leetcode id=1260 lang=rust
 *
 * [1260] Shift 2D Grid
 */

// @lc code=start
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut g = grid.clone();
        let r = grid.len();
        let c = grid[0].len();
        let n = r * c;
        let k = (k as usize % n) as usize;
        for i in 0..r {
            for j in 0..c { 
                let p = i*c+j+k;
                let x = (p/c)%r;
                let y = p%c;
                g[x][y]=grid[i][j];
            }
        }
        g
    }
}
// @lc code=end
