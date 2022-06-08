/*
 * @lc app=leetcode id=784 lang=rust
 *
 * [784] Letter Case Permutation
 */

// @lc code=start
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        ans.push(String::new());
        for c in s.chars(){
            let n = ans.len();
             if c.is_ascii_alphabetic(){
                for i in 0..n{
                    ans.push(ans[i].clone());
                    ans[i].push(c.to_ascii_lowercase());
                    ans[n+i].push(c.to_ascii_uppercase());
                }
            }else{
                 for i in 0..n{
                    ans[i].push(c);
                }
            }
        }
        ans
    }
}
// @lc code=end

