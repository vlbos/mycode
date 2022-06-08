/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len()<=1{
            return true;
        } 
        let mut max = nums[0] as usize;
        for (i, n) in nums.iter().enumerate() {
            if i>max{
                return false;
            }
            max = max.max(i + *n as usize);
            if max >= nums.len() - 1 {
                return true;
            }
        }
        false
    }
}
// @lc code=end
