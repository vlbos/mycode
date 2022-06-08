/*
 * @lc app=leetcode id=1493 lang=rust
 *
 * [1493] Longest Subarray of 1's After Deleting One Element
 */

// @lc code=start
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut p0, mut p1) = (0, 0);
        for &n in &nums {
            if n == 0 {
                p1 = p0;
                p0 = 0;
            } else {
                p1 += 1;
                p0 += 1;
            }
            ans = ans.max(p1);
        }
        if ans == nums.len() as i32 {
            ans - 1
        } else {
            ans
        }
    }
}
// @lc code=end
