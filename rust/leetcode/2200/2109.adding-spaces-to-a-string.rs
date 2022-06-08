/*
 * @lc app=leetcode id=2109 lang=rust
 *
 * [2109] Adding Spaces to a String
 */

// @lc code=start
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = String::new();
        let mut j =0;
        for (i,c) in s.chars().enumerate() {
           if j<spaces.len() && spaces[j]==i as i32{
                ans.push(' ');
                j+=1;
            }
            ans.push(c);
        }
        ans
    }
}
// @lc code=end
