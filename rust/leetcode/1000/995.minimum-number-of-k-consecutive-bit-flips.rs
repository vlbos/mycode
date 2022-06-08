/*
 * @lc app=leetcode id=995 lang=rust
 *
 * [995] Minimum Number of K Consecutive Bit Flips
 */

// @lc code=start
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut ans = 0;
        let mut rev_cnt = 0;
        let mut nums = nums;
        for i in 0..n {
            if i >= k && nums[i - k] > 1 {
                rev_cnt ^= 1;
                nums[i - k] -= 2;
            }
            if nums[i] == rev_cnt {
                if i + k > n {
                    return -1;
                }
                ans += 1;
                rev_cnt ^= 1;
                nums[i] += 2;
            }
        }
        ans
    }
}
// @lc code=end
