/*
 * @lc app=leetcode id=1771 lang=rust
 *
 * [1771] Maximize Palindrome Length From Subsequences
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let (l1, l2) = (word1.len(), word2.len());
        let n = l1 + l2;
        let word = word1 + word2.as_str();
        let mut dp = vec![vec![0; n]; n];
        let w = word.as_bytes();
        let mut ans = if w[l1 - 1] == w[l1] { 2 } else { 0 };
        for i in 0..n {
            dp[i][i] = 1;
        }
        for i in 0..n - 1 {
            dp[i][i + 1] = if w[i] == w[i + 1] { 2 } else { 1 };
        }
        for l in 2..n {
            for i in 0..n - l {
                let j = i + l;
                if w[i] == w[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                    if i < l1 && j >= l1 {
                        ans = ans.max(dp[i][j]);
                    }
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i + 1][j]);
                }
            }
        }
       
        ans
    }
}
// @lc code=end
