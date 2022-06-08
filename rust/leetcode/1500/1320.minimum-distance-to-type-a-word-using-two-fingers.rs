/*
 * @lc app=leetcode id=1320 lang=rust
 *
 * [1320] Minimum Distance to Type a Word Using Two Fingers
 */

// @lc code=start
impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let n = word.len();
        let w = word.as_bytes();
        let mut dp = vec![vec![i32::MAX/3; 26]; n];
        dp[0]=vec![0;26];
        let get_distance = |p: i32, q: i32| {
            let (x1, y1, x2, y2) = (p / 6, p % 6, q / 6, q % 6);
            (x1 - x2).abs() + (y1 - y2).abs()
        };
        for i in 1..n {
            let (cur, prev) = ((w[i] - b'A') as usize, (w[i - 1] - b'A') as usize);
            let d = get_distance(prev as i32, cur as i32);
            for j in 0..26 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + d);

                if prev != j {
                    continue;
                }
                for k in 0..26 {
                    let d0 = get_distance(k as i32, cur as i32);
                    dp[i][j] = dp[i][j].min(dp[i - 1][k] + d0);
                }
            }
        }
      
        dp.remove(n-1)
            .into_iter()
            .min()
            .unwrap()
    }
}
// @lc code=end
