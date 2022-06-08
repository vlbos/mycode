/*
 * @lc app=leetcode id=1764 lang=rust
 *
 * [1764] Form Array by Concatenating Subarrays of Another Array
 */

// @lc code=start
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;
        for g in &groups {
            let mut flag = false;
            while i + g.len() <= nums.len() {
                if g == &nums[i..i + g.len()] {
                    flag = true;
                    i += g.len();
                    break;
                } else {
                    i += 1;
                }
            }
            if !flag {
                return false;
            }
        }
        true
    }
}
// @lc code=end
