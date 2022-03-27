/*
 * @lc app=leetcode id=2149 lang=rust
 *
 * [2149] Rearrange Array Elements by Sign
 */

// @lc code=start
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let (mut pos, mut neg) = (0, 0);
        for _ in 0..(nums.len() / 2) {
            while nums[pos] < 0 {
                pos += 1;
            }
            ans.push(nums[pos]);
            pos += 1;
            while nums[neg] > 0 {
                neg += 1;
            }
            ans.push(nums[neg]);
            neg += 1;
        }
        ans
    }
}
// @lc code=end
