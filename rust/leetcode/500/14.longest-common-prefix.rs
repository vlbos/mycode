/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
         if (strs.len() == 1) {
            return (*strs.get(0).unwrap()).clone();
        }
        let mut i = usize::MAX;
        i = strs.iter().map(|v| v.len()).min().unwrap();
        while i > 0 {
            let substr = &strs.get(0).unwrap()[0..i];
            let mut r = strs.iter().filter(|v| substr != &v[0..i]);
            if r.next() == None {
                return String::from(substr);
            }

            i -= 1;
        }
        String::from("")
    }
}
// @lc code=end

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut trie=Trie::new();
        for str in &strs{
            trie.insert(str);
        }
        trie.find()
    }
}
use std::collections::HashMap;
struct Trie{
children:HashMap<char,Trie>,
is_end:bool,
}

impl Trie{
  fn new()->Self{
      Self{children:HashMap::new(),is_end:false}
  }
  fn insert(&mut self,word:&String){
      let mut node=self;
      for c in word.chars(){
          node=node.children.entry(c).or_insert(Trie::new());
      }
      node.is_end=true;
  }
  fn find(&self)->String{
      let mut node=self;
      let mut ans=String::new();
      if node.children.len()>1||node.is_end{
                return ans
            }
      while let Some((&c,children))=node.children.iter().next()
      {
           ans.push(c);
            node=children;
              if node.children.len()>1||node.is_end{
                break
            }
      }
      ans
  }
}