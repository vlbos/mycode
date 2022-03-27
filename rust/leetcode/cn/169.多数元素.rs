/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let mut len = 1;
        for j in 1..nums.len() {
            if nums[i] == nums[j] {
                len += 1;
            } else {
                len -= 1;
                if len <= 0 {
                    len = 1;
                    i = j;
                }
            }
        }
        nums[i]
    }
}
// @lc code=end
