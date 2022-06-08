/*
 * @lc app=leetcode id=451 lang=rust
 *
 * [451] Sort Characters By Frequency
 */

// @lc code=start
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut m = std::collections::HashMap::new();
        for c in s.chars(){
            *m.entry(c).or_insert(0)+=1;
        }
        let mut mv = m.iter().map(|(k,v)| (*k,*v)).collect::<Vec<(char,i32)>>();
        mv.sort_by(|a,b| b.1.cmp(&a.1));
        let mut ans = String::new();
        for k in &mv{
            ans.push_str(&(k.0.to_string()).repeat(k.1 as usize));
        }
        ans
    }
}
// @lc code=end

