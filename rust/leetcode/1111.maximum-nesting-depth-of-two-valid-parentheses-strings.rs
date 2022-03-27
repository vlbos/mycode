/*
 * @lc app=leetcode id=1111 lang=rust
 *
 * [1111] Maximum Nesting Depth of Two Valid Parentheses Strings
 */

// @lc code=start
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut ans = Vec::new();
        for (i,c) in seq.chars().enumerate(){
            let i = i as i32;
            ans.push( if c=='('{i&1}else{1-i&1});
        }
        ans
    }
}
// @lc code=end

