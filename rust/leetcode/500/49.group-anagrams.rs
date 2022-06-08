/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len()<2{
            return vec![strs];
        }
        let mut m = std::collections::HashMap::new();
        let mut a =vec![0;26];
        for s in &strs{
            a =vec![0;26];
            for c in s.chars(){
                 a[(c as u8-'a' as u8) as usize]+=1;
            }
            let mut b = m.entry(a.clone()).or_insert(Vec::new());
            b.push((*s).clone());
        }
        let mut ans = Vec::new();
        for (_,v) in &m{
            ans.push((*v).clone());
        }
        ans
    }
}
// @lc code=end

