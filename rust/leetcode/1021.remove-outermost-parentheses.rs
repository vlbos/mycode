/*
 * @lc app=leetcode id=1021 lang=rust
 *
 * [1021] Remove Outermost Parentheses
 */

// @lc code=start
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut r = String::new();
        let mut i = 0;
        let mut j = 0;
        for c in s.chars(){
            if c=='('{
                i+=1;
                if i>1{
                    r.push(c);
                }
            }else {
                j+=1;
                if i==j{
                    i=0;
                    j=0;
                }else{
                    r.push(c); 
                }
            }
        }
        r
    }
}
// @lc code=end

