/*
 * @lc app=leetcode id=31 lang=rust
 *
 * [31] Next Permutation
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let mut i = nums.len() - 2;
        let mut flag = true;
        while i >= 0 && nums[i] >= nums[i + 1] {
            if i == 0 {
                flag = false;
                break;
            }
            i -= 1;
        }
        if flag {
            let mut j = nums.len() - 1;
            while j >= 0 && nums[i] >= nums[j] {
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            let t = nums[i];
            nums[i] = nums[j];
            nums[j] = t;
        }
        if flag {
            nums[i + 1..].sort();
        } else {
            nums.sort();
        }
    }
}
// @lc code=end
