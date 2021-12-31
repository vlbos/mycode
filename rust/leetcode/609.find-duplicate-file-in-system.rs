/*
 * @lc app=leetcode id=609 lang=rust
 *
 * [609] Find Duplicate File in System
 */

// @lc code=start
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = std::collections::HashMap::new();
        for  x in &paths{
            let s = x.split_ascii_whitespace().collect::<Vec<&str>>();
            if s.len() > 1 {
                for xx in &s[1..]{
                    let  ss = xx.split('(').collect::<Vec<&str>>();
                    let mut s2 = ss[1].chars().collect::<Vec<char>>();
                    s2.pop();
                    let sss=s2.iter().collect::<String>();
                    hm.entry(sss.clone()).or_insert(vec![]).push(format!("{}/{}",s[0],ss[0].clone()));
                }
            }
        }
        hm.iter().filter(|x|x.1.len()>1).map(|x|x.1.clone()).collect::<Vec<Vec<String>>>()
    }
}
// @lc code=end
