/*
 * @lc app=leetcode id=719 lang=rust
 *
 * [719] Find K-th Smallest Pair Distance
 */

// @lc code=start
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let possible = |guess: i32| {
            let mut count = 0;
            let mut left = 0;
            for (right, &num) in nums.iter().enumerate() {
                while num - nums[left] > guess {
                    left += 1;
                }
                count += right - left;
            }
            (count as i32)>= k
        };
        let mut lo = 0;
        let mut hi = nums[nums.len() - 1] - nums[0];
        while lo < hi {
            let mid = (lo + hi) / 2;
            if possible(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}
// @lc code=end
