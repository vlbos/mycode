/*
 * @lc app=leetcode id=2008 lang=rust
 *
 * [2008] Maximum Earnings From Taxi
 */

// @lc code=start
impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
       let n1 = n as usize + 1;
        let mut f = vec![0; n1];
        let mut g = vec![Vec::new(); n1];
        for r in &rides {
            let e = r[1] as usize;
            g[e].push(vec![r[0] as i64, r[2] as i64] );
        }
        for e in 1..n1 {
            f[e] = f[e - 1];
            for r in &g[e] {
                let s = r[0] as usize;
                f[e] = f[e].max(f[s] + e as i64 - r[0] + r[1]);
            }
        }
        f[n1 - 1]
    }
}
// @lc code=end
