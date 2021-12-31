/*
 * @lc app=leetcode id=1437 lang=rust
 *
 * [1437] Check If All 1's Are at Least Length K Places Away
 */

// @lc code=start
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut pre = nums.len();
        for (i, n) in nums.iter().enumerate() {
            if *n == 1 {
                if pre < nums.len() && i - pre <= k as usize{
                    return false;
                } 
                pre =i;
            }
        }
        true
    }
}
// @lc code=end
