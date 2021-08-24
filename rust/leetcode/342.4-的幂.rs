/*
 * @lc app=leetcode.cn id=342 lang=rust
 *
 * [342] 4的幂
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
      n>0 && n&(n-1)==0 && n&0x2aaaaaaa==0
    }
}
// @lc code=end

