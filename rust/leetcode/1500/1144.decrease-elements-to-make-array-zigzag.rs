/*
 * @lc app=leetcode id=1144 lang=rust
 *
 * [1144] Decrease Elements To Make Array Zigzag
 */

// @lc code=start
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut ans = [0, 0];
        for i in 0..nums.len() {
            let index = i as usize & 1;
            ans[index] += (if i > 0 && nums[i] >= nums[i - 1] {
                nums[i] - nums[i - 1] + 1
            } else {
                0
            })
            .max(if i < nums.len() - 1 && nums[i] >= nums[i + 1] {
                nums[i] - nums[i + 1] + 1
            } else {
                0
            });
        }
        ans[0].min(ans[1])
    }
}
// @lc code=end
