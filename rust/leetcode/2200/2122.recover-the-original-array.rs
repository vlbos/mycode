/*
 * @lc app=leetcode id=2122 lang=rust
 *
 * [2122] Recover the Original Array
 */

// @lc code=start
impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        for i in 1..n {
            if nums[i] == nums[0] || (nums[i] - nums[0]) % 2 > 0 {
                continue;
            }
            let mut used = vec![false; n];
            used[0] = true;
            used[i] = true;
            let k = (nums[i] - nums[0]) / 2;
            let mut ans = vec![nums[0] + k];
            let (mut left, mut right) = (0, i);
            for j in 2..=n / 2 {
                while used[left] {
                    left += 1;
                }
                while right < n && (used[right] || nums[right] - nums[left] != k * 2) {
                    right += 1;
                }
                if right == n {
                    break;
                }
                ans.push(nums[left] + k);
                used[left] = true;
                used[right] = true;
            }
            if ans.len() == n / 2 {
                return ans;
            }
        }
        Vec::new()
    }
}
// @lc code=end
