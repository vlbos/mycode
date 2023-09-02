/*
 * @lc app=leetcode id=336 lang=rust
 *
 * [336] Palindrome Pairs
 */

// @lc code=start
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
       use std::collections::HashMap;
        let w2i: HashMap<String, usize> = words
            .iter()
            .enumerate()
            .map(|(i, s)| (s.chars().rev().collect(), i))
            .collect();
        let is_palindrome = |s: &String, left: usize, right: usize| -> bool {
            let bs = s.as_bytes();
            let len = if left > right { 0 } else { right - left + 1 };
            for i in 0..len / 2 {
                if bs[left + i] != bs[right - i] {
                    return false;
                }
            }
            true
        };
        let mut ans = Vec::new();
        for (i, word) in words.iter().enumerate() {
            if word.is_empty() {
                continue;
            }
            let m = word.len();
            for j in 0..=m {
                if is_palindrome(word, j, m - 1) {
                    let rs = if j>0{(word[..=j - 1]).to_string()}else{String::new()};
                    if let Some(&k) = w2i.get(&rs) {
                        if k != i {
                            ans.push(vec![i as i32, k as i32]);
                        }
                    }
                }
                if j > 0 && is_palindrome(word, 0, j - 1) {
                    if let Some(&k) = w2i.get(&(word[j..=m - 1]).to_string()) {
                        if k != i {
                            ans.push(vec![k as i32, i as i32]);
                        }
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie=Trie::new();
        let is_palindrome=|bw:&[u8]|{
            if bw.is_empty(){
                return true
            }
            let (mut i,mut j)=(0,bw.len()-1);
            while i<j{
                if bw[i]!=bw[j]{
                    return false
                }
                i+=1;
                j-=1;
            }
            true
        };
        for (i,word) in words.iter().enumerate(){
            let i=i as i32;
            let rev:String=word.chars().rev().collect();
            let mut node=&mut trie;
            if is_palindrome(rev.as_bytes()){
                node.suffixs.push(i);
            }
            for (j,c) in rev.chars().enumerate(){
                node=node.children.entry(c).or_insert(Trie::new());
                if is_palindrome(rev[j+1..].as_bytes()){
                    node.suffixs.push(i);
                }
            }
            node.words.push(i);
        }
        let mut ans=Vec::new();
        for (i,word) in words.iter().enumerate(){
            let i=i as i32;
            let mut node=&trie;
            let mut j=0;
            let bw=word.as_bytes();
            while j<word.len(){
                if is_palindrome(word[j..].as_bytes()){
                    for &k in &node.words{
                        if k!=i{
                            ans.push(vec![i,k]);
                        }
                    }
                }
                if let Some(child)=node.children.get(&(bw[j] as char)){
                    node=child;
                }else{
                    break
                }
                j+=1;
            }
            if j==word.len(){
                 for &k in &node.suffixs{
                        if k!=i{
                            ans.push(vec![i,k]);
                        }
                }
            }
        }
        ans
    }
}
use std::collections::HashMap;
struct Trie{
    children:HashMap<char,Trie>,
    words:Vec<i32>,
    suffixs:Vec<i32>,
}
impl Trie{
    fn new()->Self{
        Self{children:HashMap::new(),words:Vec::new(),suffixs:Vec::new()}
    }
}