/*
 * @lc app=leetcode id=724 lang=rust
 *
 * [724] Find Pivot Index
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for n in &nums {
            sum += *n;
        }
        let mut lsum = 0;
        let mut rsum = sum - nums[0];
        if lsum == rsum {
            return 0;
        }
        for i in 1..nums.len() {
            lsum += nums[i - 1];
            rsum -= nums[i];
            if lsum == rsum {
                return i as i32;
            }
        }

        -1
    }
}
// @lc code=end

