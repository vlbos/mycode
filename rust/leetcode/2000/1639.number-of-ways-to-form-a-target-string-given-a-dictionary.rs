/*
 * @lc app=leetcode id=1639 lang=rust
 *
 * [1639] Number of Ways to Form a Target String Given a Dictionary
 */

// @lc code=start
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let n = words[0].len();
        let mut cnt = vec![vec![0; 26]; n];
        for w in &words {
            for (i, b) in w.bytes().enumerate() {
                cnt[i][(b - b'a') as usize] += 1;
            }
        }
        let m = target.len();
        let mut dp = vec![vec![-1; m]; n];
        fn dfs(
            dp: &mut Vec<Vec<i32>>,
            cnt: &Vec<Vec<i32>>,
            target: &String,
            i: usize,
            j: usize,
        ) -> i32 {
            let (n, m) = (dp.len(), dp[0].len());
            if j == m {
                return 1;
            }
            if n - i < m - j {
                return 0;
            }
            if dp[i][j] != -1 {
                return dp[i][j];
            }
            let mut val = (cnt[i][(target.as_bytes()[j] - b'a') as usize] as i64)
                * (dfs(dp, cnt, target, i + 1, j + 1) as i64)
                + (dfs(dp, cnt, target, i + 1, j) as i64);
            dp[i][j] = (val % 1_000_000_007) as i32;
            dp[i][j]
        }
        dfs(&mut dp, &cnt, &target, 0, 0)
    }
}
// @lc code=end
