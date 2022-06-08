/*
 * @lc app=leetcode id=1031 lang=rust
 *
 * [1031] Maximum Sum of Two Non-Overlapping Subarrays
 */

// @lc code=start
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let (fl, sl) = (first_len as usize, second_len as usize);
        for i in 1..n {
            nums[i] += nums[i - 1];
        }
        let mut ans = nums[fl + sl - 1];
        let (mut lmax, mut smax) = (nums[fl - 1], nums[sl - 1]);
        for i in fl + sl..n {
            lmax = lmax.max(nums[i - sl] - nums[i - fl - sl]);
            smax = smax.max(nums[i - fl] - nums[i - fl - sl]);
            ans = ans.max((lmax + nums[i] - nums[i - sl]).max(smax + nums[i] - nums[i - fl]));
        }
        ans
    }
}
// @lc code=end
