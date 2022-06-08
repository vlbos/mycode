/*
 * @lc app=leetcode id=1680 lang=rust
 *
 * [1680] Concatenation of Consecutive Binary Numbers
 */

// @lc code=start
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
       let mut ans = 0i64;
        let mut shift = 0;
        for i in 1..=n {
            if (i & (i - 1)) == 0 {
                shift += 1;
            }
            ans = ((ans << shift )% 1000_000_007+ i as i64) % 1000_000_007;
        }
        ans as _
    }
}
// @lc code=end
