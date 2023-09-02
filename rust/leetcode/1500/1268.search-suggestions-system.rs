/*
 * @lc app=leetcode id=1268 lang=rust
 *
 * [1268] Search Suggestions System
 */

// @lc code=start
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let mut query = String::new();
        let mut ans = Vec::new();
        for c in search_word.chars() {
            query.push(c);
            if let Ok(i) | Err(i) = products.binary_search(&query) {
                let mut selects = Vec::new();
                for j in 0..3 {
                    if i + j < products.len() && products[i + j].starts_with(&query) {
                        selects.push(products[i + j].clone());
                    }
                }
                ans.push(selects);
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        use std::collections::{HashMap,BinaryHeap};
        #[derive(Default)]
        struct Trie{
            children:HashMap<char,Trie>,
            words:BinaryHeap<String>,
        }
        let mut trie=Trie::default();
        for p in &products{
            let mut node=&mut trie;
            for c in p.chars(){
                node=node.children.entry(c).or_insert(Trie::default());
                node.words.push(p.clone());
                if node.words.len()>3{
                    node.words.pop();
                }
            }
        }
        let mut ans=Vec::new();
          let mut node=&trie;
          for (i,c) in search_word.chars().enumerate(){
            if let Some(child)=node.children.get(&c){
                node=child;
                 ans.push(node.words.clone().into_sorted_vec());
            }else{ans.extend(vec![vec![];search_word.len()-i]);
            break}
          }
        ans
    }
}