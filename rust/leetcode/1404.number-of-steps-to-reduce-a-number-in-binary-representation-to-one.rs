/*
 * @lc app=leetcode id=1404 lang=rust
 *
 * [1404] Number of Steps to Reduce a Number in Binary Representation to One
 */

// @lc code=start
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut ans = 0;
        let mut meet1 = false;
        for i in (0..bs.len()).rev() {
            if bs[i] == b'0' {
                ans += if meet1 { 2 } else { 1 };
            } else {
                if !meet1 {
                    if i!=0{
                    ans += 2;
                    }
                    meet1 = true;
                } else {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
