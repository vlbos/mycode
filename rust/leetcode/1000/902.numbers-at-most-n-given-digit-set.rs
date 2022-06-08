/*
 * @lc app=leetcode id=902 lang=rust
 *
 * [902] Numbers At Most N Given Digit Set
 */

// @lc code=start
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let s = n.to_string();
        let k = s.len();
        let mut dp = vec![0; k + 1];
        dp[k] = 1;
        let d = digits.len();
        let bs = s.as_bytes();
        for i in (0..k).rev() {
            let si = bs[i] - b'0';
            for digit in &digits {
                if digit.parse::<u8>().unwrap() < si {
                    dp[i] += d.pow((k - i - 1) as u32);
                } else if digit.parse::<u8>().unwrap() == si {
                    dp[i] += dp[i + 1];
                }
            }
        }
        for i in 1..k {
            dp[0] += d.pow(i as u32);
        }
        dp[0] as _
    }
}
// @lc code=end
