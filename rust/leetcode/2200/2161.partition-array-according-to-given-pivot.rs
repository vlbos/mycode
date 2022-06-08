/*
 * @lc app=leetcode id=2161 lang=rust
 *
 * [2161] Partition Array According to Given Pivot
 */

// @lc code=start
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut eq = Vec::new();
        let mut gt = Vec::new();
        for &v in &nums {
            if v < pivot {
                ans.push(v);
            } else if v == pivot {
                eq.push(v);
            } else {
                gt.push(v);
            }
        }
        ans.extend(eq);
        ans.extend(gt);
        ans
    }
}
// @lc code=end
