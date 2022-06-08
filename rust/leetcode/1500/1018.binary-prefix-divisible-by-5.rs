/*
 * @lc app=leetcode id=1018 lang=rust
 *
 * [1018] Binary Prefix Divisible By 5
 */

// @lc code=start
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut r = Vec::new();
        let mut v = 0i64;
        for  n in &nums{
            v *=2;
            v+= *n as i64;
            r.push(v%5==0);
            v %=5;
        }
        r
    }
}
// @lc code=end

