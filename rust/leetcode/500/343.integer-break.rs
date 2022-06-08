/*
 * @lc app=leetcode id=343 lang=rust
 *
 * [343] Integer Break
 */

// @lc code=start
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n<=3{
        return n-1;
        }
        let q =( n/3) as u32;
        let r = n%3;
        if r==0{
            3i32.pow(q )
        }else if r==1{
            3i32.pow(q-1)*4
        }else{
        3i32.pow(q)*2}
    }
}
// @lc code=end

