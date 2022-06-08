/*
 * @lc app=leetcode id=1967 lang=rust
 *
 * [1967] Number of Strings That Appear as Substrings in Word
 */

// @lc code=start
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;
        for p in &patterns{
                if word.find(p).is_some(){
                    ans+=1;
                }
        }
        ans
    }
}
// @lc code=end

