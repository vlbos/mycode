/*
 * @lc app=leetcode id=1629 lang=rust
 *
 * [1629] Slowest Key
 */

// @lc code=start
impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut cc = keys_pressed.chars().nth(0).unwrap();
        let mut max = release_times[0];
        for i in 1..release_times.len() {
            if release_times[i] - release_times[i - 1] > max
                || release_times[i] - release_times[i - 1] == max
                    && keys_pressed.chars().nth(i).unwrap() > cc
            {
                max = release_times[i] - release_times[i - 1];
                cc = keys_pressed.chars().nth(i).unwrap();
            }
        }
        cc
    }
}
// @lc code=end
