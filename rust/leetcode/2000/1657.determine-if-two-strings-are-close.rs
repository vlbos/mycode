/*
 * @lc app=leetcode id=1657 lang=rust
 *
 * [1657] Determine if Two Strings Are Close
 */

// @lc code=start
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
       use std::collections::HashMap;
        let (mut w1,mut w2)=(HashMap::new(),HashMap::new());        
        for b in word1.bytes(){
            *w1.entry(b-b'a').or_insert(0)+=1;
        }
        for b in word2.bytes(){
            *w2.entry(b-b'a').or_insert(0)+=1;
        }
        let mut f1:Vec<i32> = w1.iter().map(|x|*x.1).collect();
        let mut f2:Vec<i32> = w2.iter().map(|x|*x.1).collect();
        let mut a1:Vec<u8> = w1.iter().map(|x|*x.0).collect();
        let mut a2:Vec<u8> = w2.iter().map(|x|*x.0).collect();
        a1.sort();
        a2.sort();
        f1.sort();
        f2.sort();
        a1==a2 && f1==f2
    }
}
// @lc code=end

