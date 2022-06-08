/*
 * @lc app=leetcode id=1408 lang=rust
 *
 * [1408] String Matching in an Array
 */

// @lc code=start
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let s = words.join(" ");
        let ss =|w:String|->bool{ s.matches(&w).collect::<Vec<&str>>().len() >= 2};
        words.iter().filter(|&w|ss((*w).clone())).map(|w|w.to_string()).collect::<Vec<String>>()
    }
}
// @lc code=end
