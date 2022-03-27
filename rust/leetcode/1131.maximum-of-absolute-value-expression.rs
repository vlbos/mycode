/*
 * @lc app=leetcode id=1131 lang=rust
 *
 * [1131] Maximum of Absolute Value Expression
 */

// @lc code=start
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (dx, dy) in &[(1, 1), (-1, 1), (1, -1), (-1, -1)] {
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            for i in 0..arr1.len() {
                max = max.max(arr1[i] * dx + arr2[i] * dy + i as i32);
                min = min.min(arr1[i] * dx + arr2[i] * dy + i as i32);
            }
            ans = ans.max(max - min);
        }
        ans
    }
}
// @lc code=end
