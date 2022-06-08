/*
 * @lc app=leetcode id=1760 lang=rust
 *
 * [1760] Minimum Limit of Balls in a Bag
 */

// @lc code=start
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let is_valid = |res: i32| -> bool {
            let mut ans = 0;
            for &n in &nums {
                ans += (n - 1) / res;
                if ans > max_operations {
                    return false;
                }
            }
            true
        };
        let (mut l, mut r) = (1, nums[nums.len() - 1]);
        let mut ans = nums[0];
        while l <= r {
            let mid = (l + r) / 2;
            if is_valid(mid) {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}
// @lc code=end
