/*
 * @lc app=leetcode id=1859 lang=rust
 *
 * [1859] Sorting the Sentence
 */

// @lc code=start
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut v = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut ans = vec!["";v.len()];
        for j in 0..v.len(){
            let ss = v[j];
            let i = (ss.chars().nth(ss.len()-1).unwrap() as u8-'1' as u8) as usize;
            ans[i]= &ss[..ss.len()-1];
        }
        ans.join(" ")
    }
}
// @lc code=end

