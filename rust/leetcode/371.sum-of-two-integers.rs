/*
 * @lc app=leetcode id=371 lang=rust
 *
 * [371] Sum of Two Integers
 */

// @lc code=start
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c =0;
        while b!=0{
            c = (a&b)<<1;
            a=a^b;
            b=c;
        }
        a
    }
}
// @lc code=end

