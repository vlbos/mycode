/*
 * @lc app=leetcode.cn id=453 lang=rust
 *
 * [453] 最小操作次数使数组元素相等
 */

// @lc code=start
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut moves = 0;
        let mut min = i32::MAX;
        for n in &nums {
            min = min.min(*n);
        }
        for n in &nums {
            moves += *n - min;
        }
        moves
    }
}
// @lc code=end
