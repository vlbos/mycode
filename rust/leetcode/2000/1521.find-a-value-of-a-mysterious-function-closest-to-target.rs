/*
 * @lc app=leetcode id=1521 lang=rust
 *
 * [1521] Find a Value of a Mysterious Function Closest to Target
 */

// @lc code=start
impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut ans = (arr[0] - target).abs();
        let mut valid = vec![arr[0]];
        for &num in &arr {
            let mut new_valid = vec![num];
            ans = ans.min((num - target).abs());
            for &prev in &valid {
                new_valid.push(prev & num);
                ans = ans.min(((prev&num) - target).abs());
            }
            new_valid.dedup();
            valid = new_valid;
        }
        ans
    }
}
// @lc code=end
