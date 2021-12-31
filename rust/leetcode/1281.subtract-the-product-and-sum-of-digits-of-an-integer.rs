/*
 * @lc app=leetcode id=1281 lang=rust
 *
 * [1281] Subtract the Product and Sum of Digits of an Integer
 */

// @lc code=start
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut p = 1;
        let mut s = 0;
        while n>0{
              let nn = n%10;
              p*=nn;
              s+=nn;
              n/=10;
        }
        p-s
    }
}
// @lc code=end

