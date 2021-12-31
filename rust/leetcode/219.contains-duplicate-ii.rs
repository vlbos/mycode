/*
 * @lc app=leetcode id=219 lang=rust
 *
 * [219] Contains Duplicate II
 */

// @lc code=start
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = m.get_mut(n) {
                if (i as i32 - *j).abs() <= k {
                    return true;
                } else {
                    *j = i as i32;
                }
            } else {
                m.insert(*n, i as i32);
            }
        }
        false
    }
}
// @lc code=end

