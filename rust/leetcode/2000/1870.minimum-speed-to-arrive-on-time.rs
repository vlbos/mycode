/*
 * @lc app=leetcode id=1870 lang=rust
 *
 * [1870] Minimum Speed to Arrive on Time
 */

// @lc code=start
impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let n = dist.len();
        let hr = (hour * 100.00).round() as i64;
        if hr <= (n as i64 - 1) * 100 {
            return -1;
        }
        let (mut l, mut r) = (1, 10_000_000);
        while l < r {
            let mid = (l + r) / 2;
            let mut t = 0;
            for i in 0..n - 1 {
                t += (dist[i] as i64 - 1) / mid + 1;
            }
            t *= mid;
            t += dist[n - 1] as i64;
            if t * 100 <= hr * mid {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as _
    }
}
// @lc code=end
