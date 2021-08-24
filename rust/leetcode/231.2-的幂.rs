/*
 * @lc app=leetcode.cn id=231 lang=rust
 *
 * [231] 2 的幂
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n>0 && (n&(n-1))==0
        //  n>0 && (n&(-n))==n
        // n>0 && (n^(n-1))==(n*2-1)
        // n>0 && (1<<30)%n==0
    }
}
// @lc code=end

 