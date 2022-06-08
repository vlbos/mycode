/*
 * @lc app=leetcode id=1528 lang=rust
 *
 * [1528] Shuffle String
 */

// @lc code=start
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut sv = s.chars().collect::<Vec<char>>();
        let mut ans= sv.clone();
        for i in 0..sv.len(){
            ans[indices[i] as usize]=sv[i];
        }
        ans.iter().collect()
    }
}
// @lc code=end

