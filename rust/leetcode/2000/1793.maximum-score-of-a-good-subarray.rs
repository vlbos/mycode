/*
 * @lc app=leetcode id=1793 lang=rust
 *
 * [1793] Maximum Score of a Good Subarray
 */

// @lc code=start
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32;
        let (mut l, mut r) = (k, k);
        let mut ans = 0;
        let mut nums=nums;
        loop {
            let nk = nums[k as usize];
            while r < n && nums[r as usize] >= nk {
                r += 1;
            }
            while l >= 0 && nums[l as usize] >= nk {
                l -= 1;
            }
            ans = ans.max((r - l - 1) * nk);
            if l < 0 && r == n {
                break;
            }
            if l >= 0 && r < n {
                nums[k as usize] = nums[l as usize].max(nums[r as usize]);
            } else if l < 0 {
                nums[k as usize] = nums[r as usize];
            } else {
                nums[k as usize] = nums[l as usize];
            }
        }
        ans
    }
}
// @lc code=end
