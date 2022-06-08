/*
 * @lc app=leetcode id=397 lang=rust
 *
 * [397] Integer Replacement
 */

// @lc code=start
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut ans = 0;
        let mut nn = n as i64;
        while nn > 1 {
            if nn & 1 == 0 {
                nn >>= 1;
            } else if nn&2 == 0||nn==3 {
                nn -= 1;
            } else {
                nn += 1;
            }
            ans += 1;
        }
        ans
    }
}
// @lc code=end
