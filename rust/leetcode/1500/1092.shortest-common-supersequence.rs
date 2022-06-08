/*
 * @lc app=leetcode id=1092 lang=rust
 *
 * [1092] Shortest Common Supersequence
 */

// @lc code=start
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
         let mut ans = Vec::new();
        let (m, n) = (str1.len(), str2.len());
        let (s1, s2) = (str1.as_bytes(), str2.as_bytes());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if s1[i-1] == s2[j-1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        let (mut i, mut j) = (m, n);
        while dp[i][j] > 0 {
            if s1[i-1] == s2[j-1] {
                ans.push(s1[i-1]);
                i -= 1;
                j -= 1;
            } else {
                if dp[i - 1][j] >= dp[i][j - 1] {
                    ans.push(s1[i-1]);
                    i -= 1;
                } else {
                    ans.push(s2[j-1]);

                    j -= 1;
                }
            }
           
        }
        while i > 0 {
            ans.push(s1[i-1]);
            i -= 1;
        }
        while j > 0 {
            ans.push(s2[j-1]);
            j -= 1;
        }
        ans.reverse();
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end
