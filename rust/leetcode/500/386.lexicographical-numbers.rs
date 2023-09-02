/*
 * @lc app=leetcode id=386 lang=rust
 *
 * [386] Lexicographical Numbers
 */

// @lc code=start
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut nnn = 1;
        while ans.len() < n as usize {
            while nnn <= n {
                ans.push(nnn);
                nnn *= 10;
            }
            while nnn % 10 == 9 || nnn > n {
                nnn /= 10;
            }
            nnn += 1;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut trie=Trie::new();
        let mut ans=Vec::new();
        trie.inserts(n,0,&mut ans);
        ans
    }
}
use std::collections::HashMap;
struct Trie{
    children:HashMap<i32,Trie>,
    cur:i32,
}
impl Trie{
    fn new()->Self{
        Self{children:HashMap::new(),cur:0}
    }
    fn inserts(&mut self,n:i32,prev:i32,ans:&mut Vec<i32>){
        let mut node=self;
        for i in 0..10{
            node.children.entry(i).or_insert(Trie::new()).cur=prev*10+i;
            if node.children.get(&i).unwrap().cur>n{
                return
            }
            let cur=node.children.get(&i).unwrap().cur;
             if cur!=0{
                 ans.push(cur);
                 node.children.get_mut(&i).unwrap().inserts(n,cur,ans);
             }
        }
    }
}