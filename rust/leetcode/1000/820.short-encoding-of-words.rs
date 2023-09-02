/*
 * @lc app=leetcode id=820 lang=rust
 *
 * [820] Short Encoding of Words
 */

// @lc code=start
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut ws = words.iter().collect::<std::collections::HashSet<_>>();
        for w in &words{
            for i in 1..w.len(){
                  let mut ww = w.clone().split_off(i);
                   ws.remove(&ww);
            }
        }
        ws.iter().map(|x|x.len() as i32).sum::<i32>()+ws.len() as i32
    }
}
// @lc code=end

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|w|-(w.len() as i32));
        let mut trie=Trie::new();
        let mut ans=0;
        for word in &words{
            if trie.insert(word){
                ans+=word.len() as i32+1;
            }
        }
        ans
    }
}
use std::collections::HashMap;
struct Trie{
    children:HashMap<char,Trie>,
}
impl Trie{
    fn new()->Self{
        Self{children:HashMap::new()}
    }
    fn insert(&mut self,word:&String)->bool{
        let mut node=self;
        let mut is_new=false;
        for c in word.chars().rev(){
            node=node.children.entry(c).or_insert_with(||{is_new=true;Trie::new()});
        }
        is_new
    }
}