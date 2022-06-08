/*
 * @lc app=leetcode id=1883 lang=rust
 *
 * [1883] Minimum Skips to Arrive at Meeting On Time
 */

// @lc code=start
impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();
        let mut f = vec![vec![i64::MAX / 2; n + 1]; n + 1];
        f[0][0] = 0;
        let speed = speed as i64;
        for i in 1..=n {
            for j in 0..=i {
                if j != i {
                    f[i][j] =
                        f[i][j].min(((f[i - 1][j] + dist[i - 1] as i64 - 1) / speed + 1) * speed);
                }
                if j != 0 {
                    f[i][j] = f[i][j].min(f[i - 1][j - 1] + dist[i - 1] as i64);
                }
            }
        }
        if let Some(i) = f[n].iter().position(|x| *x <= hours_before as i64 * speed) {
            i as i32
        } else {
            -1
        }
    }
}
// @lc code=end
