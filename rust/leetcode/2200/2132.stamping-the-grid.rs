/*
 * @lc app=leetcode id=2132 lang=rust
 *
 * [2132] Stamping the Grid
 */

// @lc code=start
impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut sum = vec![vec![0; n + 1]; m + 1];
        let mut diff = vec![vec![0; n + 1]; m + 1];
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + v;
            }
        }

        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v != 0 {
                    continue;
                }
                let (x, y) = (i + stamp_height as usize, j + stamp_width as usize);
                if x <= m && y <= n && sum[x][y] - sum[x][j] - sum[i][y] + sum[i][j] == 0 {
                    diff[i][j] += 1;
                    diff[i][y] -= 1;
                    diff[x][j] -= 1;
                    diff[x][y] += 1;
                }
            }
        }
        let mut cnt = vec![0; n + 1];
        let mut pre = vec![0; n + 1];
        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                cnt[j + 1] = cnt[j] + pre[j + 1] - pre[j] + diff[i][j];
                if cnt[j + 1] == 0 && v == 0 {
                    return false;
                }
            }
            std::mem::swap(&mut cnt, &mut pre);
        }
        true
    }
}
// @lc code=end
