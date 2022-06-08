/*
 * @lc app=leetcode id=1513 lang=rust
 *
 * [1513] Number of Substrings With Only 1s
 */

// @lc code=start
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut ans = 0;
        let mut count = 0;
        for c in s.chars() {
            if c == '0' {
                ans += (count as i64 * (count as i64 + 1)) % 1000_000_007 / 2;
                ans %= 1000_000_007;
                count = 0;
            } else {
                count += 1;
            }
        }
        ans += (count * (count + 1)) / 2;
        ans %= 1000_000_007;
        ans as _
    }
}
// @lc code=end
