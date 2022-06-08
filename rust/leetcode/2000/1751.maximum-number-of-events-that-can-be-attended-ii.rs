/*
 * @lc app=leetcode id=1751 lang=rust
 *
 * [1751] Maximum Number of Events That Can Be Attended II
 */

// @lc code=start
impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, k) = (events.len(), k as usize);
        let mut events = events;
        events.sort_by_key(|x| x[1]);
        let mut f = vec![vec![0; k + 1]; n + 1];
        for i in 1..=n {
            let (s, e, v) = (events[i - 1][0], events[i - 1][1], events[i - 1][2]);
            let (mut l, mut r) = (0, n);
            while l + 1 < r {
                let mid = (l + r) / 2;
                if events[mid - 1][1] < s {
                    l = mid;
                } else {
                    r = mid;
                }
            }
            for j in 1..=k {
                f[i][j] = f[i - 1][j].max(f[l][j - 1] + v);
            }
        }
        f[n][k]
    }
}
// @lc code=end
