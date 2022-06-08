/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = String::new();
        for (i,c) in word1.chars().enumerate(){
            ans.push(c);
            if i<word2.len(){
                ans.push(word2.chars().nth(i).unwrap());
                if i==word2.len()-1{
                    break;
                }
            }
        }
        if word1.len()>word2.len(){
            ans+=&((&(word1.chars().collect::<Vec<char>>())[word2.len()..]).to_vec().iter().collect::<String>());
        }else{
            ans+=&((&(word2.chars().collect::<Vec<char>>())[word1.len()..]).to_vec().iter().collect::<String>());
        }
        ans
    }
}
// @lc code=end

