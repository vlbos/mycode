/*
 * @lc app=leetcode id=1752 lang=rust
 *
 * [1752] Check if Array Is Sorted and Rotated
 */

// @lc code=start
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                j = i;
                break;
            }
        }
        if j == 0 {
            return true;
        }
        let mut n = Vec::<i32>::new();
        n.extend(&nums[j..]);
        n.extend(&nums[..j]);
        for i in 0..n.len()-1 {
            if n[i+1] < n[i] {
                return false;
            }
        }
        true
    }
}
// @lc code=end
