/*
 * @lc app=leetcode id=1608 lang=rust
 *
 * [1608] Special Array With X Elements Greater Than or Equal X
 */

// @lc code=start
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; nums.len() + 1];
        for n in &nums {
            cnt[(*n as usize).min(nums.len())] += 1;
        }
        for i in (0..cnt.len()).rev() {
            if i < cnt.len() - 1 {
                cnt[i] += cnt[i + 1];
            }
            if cnt[i] == i {
                return i as i32;
            }
        }

        -1
    }
}
// @lc code=end
