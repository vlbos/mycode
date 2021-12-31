/*
 * @lc app=leetcode id=795 lang=rust
 *
 * [795] Number of Subarrays with Bounded Maximum
 */

// @lc code=start
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let count = |nums: &Vec<i32>, bound: i32| -> i32 {
            let mut i = 0;
            let mut ans = 0;
            for n in nums {
                i = if *n <= bound { i + 1 } else { 0 };
                ans += i;
            }
            ans
        };

        count(&nums, right) - count(&nums, left - 1)
    }
}
// @lc code=end
