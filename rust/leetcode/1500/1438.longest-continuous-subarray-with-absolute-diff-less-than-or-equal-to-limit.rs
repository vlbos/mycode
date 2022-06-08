/*
 * @lc app=leetcode id=1438 lang=rust
 *
 * [1438] Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
 */

// @lc code=start
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut ans = 0;
        use std::collections::VecDeque;
        let (mut que_min, mut que_max) = (VecDeque::new(), VecDeque::new());
        while r < nums.len() {
            while !que_max.is_empty() && *que_max.back().unwrap() < nums[r] {
                que_max.pop_back();
            }
            while !que_min.is_empty() && *que_min.back().unwrap() > nums[r] {
                que_min.pop_back();
            }
            que_max.push_back(nums[r]);
            que_min.push_back(nums[r]);
            while !que_max.is_empty()
                && !que_min.is_empty()
                && *que_max.front().unwrap() - *que_min.front().unwrap() > limit
            {
                if *que_max.front().unwrap() == nums[l] {
                    que_max.pop_front();
                }
                if *que_min.front().unwrap() == nums[l] {
                    que_min.pop_front();
                }
                l += 1;
            }
            ans = ans.max(r - l + 1);
            r += 1;
        }
        ans as _
    }
}
// @lc code=end
