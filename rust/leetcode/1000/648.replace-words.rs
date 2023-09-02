/*
 * @lc app=leetcode id=648 lang=rust
 *
 * [648] Replace Words
 */

// @lc code=start
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let d  = dictionary.iter().collect::<std::collections::HashSet<_>>();
        let mut ans = Vec::new();
        for ss in sentence.split_ascii_whitespace(){
              let sv = ss.chars().collect::<Vec<char>>();
              let mut flag = true;
              for i in 1..sv.len(){
                    let subs = &sv[0..i].iter().collect::<String>();
                    if d.contains(&subs){
                        ans.push(subs.clone());
                        flag=false;
                        break;
                    }
              }
              if flag{
                        ans.push(ss.to_string().clone());
              }
        }
        ans.join(" ")
    }
}
// @lc code=end

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie=Trie::new();
        for word in &dictionary{
            trie.insert(word);
        }
        sentence.split_ascii_whitespace().map(|word| trie.find(word)).collect::<Vec<String>>().join(" ")
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
    fn insert(&mut self,word:&String){
        let mut node=self;
        for c in word.chars(){
            node=node.children.entry(c).or_insert(Trie::new());
        }
        node.children.entry('#').or_insert(Trie::new());
    }
    fn find(&self,word:&str)->String{
        let mut node=self;
        for (i,c) in word.chars().enumerate(){
            if node.children.get(&'#').is_some(){
                return word[..i].to_string()
            }
            if let Some(child)=node.children.get(&c){
                    node=child;

            }else{
                break
            }
        }
        word.to_string()
    }
}
