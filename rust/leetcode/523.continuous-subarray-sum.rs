/*
 * @lc app=leetcode id=523 lang=rust
 *
 * [523] Continuous Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let n = nums.len();
        let mut m = std::collections::HashMap::<i32,i32>::new();
        m.insert(0, -1);
        let mut rem = 0;
        for i in 0..n {
            rem = (rem + nums[i]) % k;
            if let Some(j) = m.get(&rem) {
                if i - (*j) as usize >= 2 {
                    return true;
                }
            } else {
                m.insert(rem, i as i32);
            }
        }
        false
    }
}
// @lc code=end
