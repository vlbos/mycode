/*
 * @lc app=leetcode.cn id=693 lang=rust
 *
 * [693] 交替位二进制数
 */

// @lc code=start
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut b = n%2;
        let mut n=n/2;
        while n>0{
            if b==n%2{
                return false;
            }
            b=n%2;
            n/=2;
        }
        true
    }
}
// @lc code=end

