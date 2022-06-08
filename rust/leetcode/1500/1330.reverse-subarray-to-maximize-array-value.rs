/*
 * @lc app=leetcode id=1330 lang=rust
 *
 * [1330] Reverse Subarray To Maximize Array Value
 */

// @lc code=start
impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let cal = |nums: &Vec<i32>| -> i32 {
            let mut m = i32::MIN;
            let mut ans = 0;
            for i in (1..n - 2).rev() {
                m = m.max(nums[i + 1].min(nums[i + 2]));
                let cur = nums[i].max(nums[i - 1]);
                if cur < m {
                    ans = ans.max((m - cur) * 2);
                }
            }
            for i in (1..n).rev() {
                ans = ans.max(0i32.max((nums[i] - nums[0]).abs() - (nums[i] - nums[i - 1]).abs()))
            }
            ans
        };
        let mut ans = 0;
        for i in 1..n {
            ans += (nums[i] - nums[i - 1]).abs();
        }
        let ans1 = cal(&nums);
        let mut nums = nums;
        nums.reverse();
        let ans2 = cal(&nums);
        ans + ans1.max(ans2)
    }
}
// @lc code=end
