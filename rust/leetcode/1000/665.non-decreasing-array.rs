/*
 * @lc app=leetcode id=665 lang=rust
 *
 * [665] Non-decreasing Array
 */

// @lc code=start
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut nums=nums;
        let mut cnt = 0;
        for i in 0..nums.len() - 1 {
            let x = nums[i];
            let y = nums[i + 1];
            if x > y {
                cnt += 1;
                if cnt > 1 {
                    return false;
                }
                if i > 0 && y < nums[i - 1] {
                    nums[i + 1] = x;
                }
            }
        }
        true
    }
}
// @lc code=end
