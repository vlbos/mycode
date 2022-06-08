/*
 * @lc app=leetcode id=1546 lang=rust
 *
 * [1546] Maximum Number of Non-Overlapping Subarrays With Sum Equals Target
 */

// @lc code=start
impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            let mut s = std::collections::HashSet::new();
            s.insert(0);
            let mut sum = 0;
            while i < n {
                sum += nums[i];
                if s.contains(&(sum - target)) {
                    ans += 1;
                    break;
                } else {
                    s.insert(sum);
                    i += 1;
                }
            }
            i += 1;
        }
        ans
    }
}
// @lc code=end
