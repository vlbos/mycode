/*
 * @lc app=leetcode id=1078 lang=rust
 *
 * [1078] Occurrences After Bigram
 */

// @lc code=start
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut r =Vec::new();
        let t = text.split_ascii_whitespace().collect::<Vec<&str>>();
        for i in 0..t.len()-2{
            if t[i]==first && t[i+1]==second && i+2<t.len(){
                r.push(t[i+2].to_string());
            }
        }
        r
    }
}
// @lc code=end

