/*
 * @lc app=leetcode id=220 lang=rust
 *
 * [220] Contains Duplicate III
 */

// @lc code=start
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        if nums.len() < 2 {
            return false;
        }
        let mut i = 0;
        let mut j = k;
        for i in 0..nums.len()-1 {
            let l = (i + k).min(nums.len() - 1);
            for j in i + 1..l + 1 {
                if (nums[j] as i64 - nums[i] as i64).abs() <= t as i64 {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
