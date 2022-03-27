/*
 * @lc app=leetcode id=2063 lang=rust
 *
 * [2063] Vowels of All Substrings
 */

// @lc code=start
impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let mut ans = 0i64;
        let n = word.len();
        let v = "aeiou".bytes().collect::<std::collections::HashSet<u8>>();
        for (i,b) in word.bytes().enumerate(){
            if v.contains(&b){  
               ans+= ((i+1) as i64)* ((n-i) as i64);
            }
        }
        ans
    }
}
// @lc code=end

