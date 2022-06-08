/*
 * @lc app=leetcode id=514 lang=rust
 *
 * [514] Freedom Trail
 */

// @lc code=start
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let (n, m) = (ring.len(), key.len());
        let mut pos = vec![Vec::new(); 26];
        for (i, b) in ring.bytes().enumerate() {
            pos[(b - b'a') as usize].push(i);
        }
        let mut dp = vec![vec![ usize::MAX / 4;n]; m];
        let bkey = key.as_bytes();
        for &i in &pos[(bkey[0] - b'a') as usize] {
            dp[0][i] = i.min(n - i) + 1;
        }
        for i in 1..m {
            for &j in &pos[(bkey[i] - b'a') as usize ]{
                for &k in &pos[(bkey[i - 1] - b'a') as usize ]{
                    let jk = ((j as i32) - k as i32).abs() as usize;
                    dp[i][j] = dp[i][j].min(dp[i - 1][k] + jk.min(n - jk) + 1);
                }
            }
        }
        *dp[m - 1].iter().min().unwrap() as _
    }
}
// @lc code=end
