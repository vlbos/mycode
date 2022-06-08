/*
 * @lc app=leetcode id=1267 lang=rust
 *
 * [1267] Count Servers that Communicate
 */

// @lc code=start
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_cnt = vec![0; grid.len()];
        let mut col_cnt = vec![0; grid[0].len()];
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    row_cnt[i] += 1;
                    col_cnt[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 && (row_cnt[i] > 1 || col_cnt[j] > 1) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
