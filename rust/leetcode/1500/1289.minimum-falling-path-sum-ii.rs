/*
 * @lc app=leetcode id=1289 lang=rust
 *
 * [1289] Minimum Falling Path Sum II
 */

// @lc code=start
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
       let n = grid.len();
        let (mut first_sum, mut first_pos, mut second_sum) = (0, n, 0);
        for i in 0..n {
            let (mut fs, mut fp, mut ss) = (i32::MAX, n, i32::MAX);
            for j in 0..n {
                let cur_sum = if j != first_pos {
                    first_sum
                } else {
                    second_sum
                } + grid[i][j];
                if cur_sum < fs {
                    ss = fs;
                    fs = cur_sum;
                    fp = j;
                } else if cur_sum < ss {
                    ss = cur_sum;
                }
            }
            first_sum = fs;
            first_pos = fp;
            second_sum = ss;
        }
        first_sum
    }
}
// @lc code=end
