/*
 * @lc app=leetcode id=1220 lang=rust
 *
 * [1220] Count Vowels Permutation
 */

// @lc code=start
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp = vec![1i64; 5];
        for _ in 0..n - 1 {
            dp = vec![
                (dp[1] + dp[2] + dp[4]) % 1_000_000_007,
                (dp[0] + dp[2]) % 1_000_000_007,
                (dp[1] + dp[3]) % 1_000_000_007,
                dp[2],
                (dp[2] + dp[3]) % 1_000_000_007,
            ];
        }
        (dp.iter().sum::<i64>() % 1_000_000_007) as _
    }
}
// @lc code=end
