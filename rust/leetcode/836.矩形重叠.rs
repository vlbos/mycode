/*
 * @lc app=leetcode.cn id=836 lang=rust
 *
 * [836] 矩形重叠
 */

// @lc code=start
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        !(rec1[0]>=rec2[2] || rec1[1]>=rec2[3]||rec2[0]>=rec1[2] || rec2[1]>=rec1[3])
    }
}
// @lc code=end

