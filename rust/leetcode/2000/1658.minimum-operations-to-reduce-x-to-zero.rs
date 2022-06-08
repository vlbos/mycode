/*
 * @lc app=leetcode id=1658 lang=rust
 *
 * [1658] Minimum Operations to Reduce X to Zero
 */

// @lc code=start
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let y = sum - x;
        let n = nums.len();
        let mut ans = n + 1;
        let (mut l, mut r) = (0, 0);
        let mut cur = 0;
        while r < n {
            cur += nums[r];
            while l <= r && cur > y {
                cur -= nums[l];
                l += 1;
            }
            if cur == y {
                ans = ans.min(n - r + l - 1);
            }
            r += 1;
        }
        if ans == n + 1 {
            -1
        } else {
            ans as _
        }
    }
}
// @lc code=end
