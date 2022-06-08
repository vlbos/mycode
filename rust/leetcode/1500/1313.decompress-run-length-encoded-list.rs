/*
 * @lc app=leetcode id=1313 lang=rust
 *
 * [1313] Decompress Run-Length Encoded List
 */

// @lc code=start
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in (0..nums.len()).step_by(2){
            ans.extend(vec![nums[i+1];nums[i] as usize]);
        }
        ans
    }
}
// @lc code=end

