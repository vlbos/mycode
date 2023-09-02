/*
 * @lc app=leetcode id=139 lang=rust
 *
 * [139] Word Break
 */

// @lc code=start
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
            let d = word_dict.iter().collect::<std::collections::HashSet<_>>();
            let sv= s.chars().collect::<Vec<char>>();
            let mut dp = vec![false;sv.len()+1];
            dp[0]=true;
            for i in 1..=sv.len(){
                    for j in 0..i{
                        let ss = sv[j..i].iter().collect::<String>();
                        if dp[j]&& d.contains(&ss){
                            dp[i]=true;
                            break;
                        }
                    }
            }
            dp[sv.len()]
    }
}
// @lc code=end

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut trie=Trie::new();
        for w in &word_dict{
            trie.insert(w);
        }
        let n=s.len();
        let mut dp=vec![false;n];
        let bs=s.as_bytes();
        for i in (0..n).rev(){
            let mut node=&trie;
            for j in i..n{
                if let Some(child)=node.children.get(&bs[j]){
                    node=child;
                }else{
                    break
                }
                if node.is_end && (j+1==n|| dp[j+1]){
                    dp[i]=true;
                    break
                }
            }
        }
        dp[0]
    }
}
use std::collections::HashMap;
struct Trie{
    children:HashMap<u8,Trie>,
    is_end:bool,
}
impl Trie{
    fn new()->Self{
        Self{children:HashMap::new(),is_end:false}
    }
    fn insert(&mut self,word:&String){
          let mut node=self;
          for c in word.bytes(){
              node=node.children.entry(c).or_insert(Trie::new());
          }
          node.is_end=true;
    }

}