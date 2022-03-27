/*
 * @lc app=leetcode.cn id=944 lang=rust
 *
 * [944] 删列造序
 */

// @lc code=start
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut r = 0;
        for i in 0..strs[0].chars().count() {
            for j in 1..strs.len() {
                if strs[j - 1].chars().nth(i).unwrap() > strs[j].chars().nth(i).unwrap() {
                    r += 1;
                    break;
                }
            }
        }
        r
    }
}
// @lc code=end
