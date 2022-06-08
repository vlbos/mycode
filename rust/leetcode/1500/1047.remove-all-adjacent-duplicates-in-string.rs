/*
 * @lc app=leetcode id=1047 lang=rust
 *
 * [1047] Remove All Adjacent Duplicates In String
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars(){
             if stack.is_empty()||stack[stack.len()-1]!=c{
                    stack.push(c);
             }else if stack[stack.len()-1]==c{
                stack.pop();
             }
        }
        stack.into_iter().collect()
    }
}
// @lc code=end
