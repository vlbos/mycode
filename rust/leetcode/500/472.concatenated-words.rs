/*
 * @lc app=leetcode id=472 lang=rust
 *
 * [472] Concatenated Words
 */

// @lc code=start
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}
impl Trie {
    fn new() -> Self {
        Self::default()
    }
    fn insert(&mut self, word: &String) {
        let mut node = self;
        for b in word.bytes() {
            let k = (b - b'a') as usize;
            node=node.children[k].get_or_insert_with(||Default::default());
        }
        node.is_end = true;
    }
    fn dfs(&self, word: &String, start: usize, vis: &mut Vec<bool>) -> bool {
        if start == word.len() {
            return true;
        }
        if vis[start] {
            return false;
        }
        vis[start] = true;
        let mut node = self;
        let bw = word.as_bytes();
        for i in start..word.len() {
            let k = (bw[i] - b'a') as usize;
            if let Some(n) = &node.children[k]
            {
                        node =n;
            }else{
                       return false;
            }
           if node.is_end && self.dfs(word, i + 1, vis) {
                return true;
            }
        }
        false
    }
}
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_by_key(|x| x.len());
        let mut ans = Vec::new();
        let mut root = Trie::new();
        for word in &words {
            if word.is_empty() {
                continue;
            }
            let mut vis = vec![false; word.len()];
            if root.dfs(word, 0, &mut vis) {
                ans.push(word.clone());
            } else {
                root.insert(word);
            }
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        words.sort_by_key(|w|w.len());
        let mut ans=Vec::new();
        let mut trie=Trie::new();

        for word in &words{
            if trie.dfs(0,word,&mut vec![false;word.len()]){
                ans.push(word.clone());
            }else{
                trie.insert(word);
            }
        }
        ans
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
    fn dfs(&self,i:usize,word:&String,vis:&mut Vec<bool>)->bool{
        if i==word.len(){
            return true
        }
        if vis[i]{
            return false
        }
        vis[i]=true;
      
        let mut node=self;
          let bw=word.as_bytes();
        for j in i..word.len(){
            if let Some(child)=node.children.get(&bw[j]){
                node=child;
            }else{
                return false
            }
            if node.is_end && self.dfs(j+1,word,vis){
                return true
            }
        }
        false
    }
}



impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        let mut set= std::collections::HashSet::new();
        let (p,offset)=(131,128);
        for word in &words{
            set.insert(word.bytes().fold(0,|mut hash,b| {hash=hash*p+offset+(b-b'a') as i64;hash}));
            }
        let check=|s:&String|{
            let n=s.len();
            let bs=s.as_bytes();
            let mut f=vec![-1;n+1];
            f[0]=0;
            for i in 0..=n{
                if f[i]==-1{
                    continue
                }
                let mut cur=0;
                for j in i+1..=n{
                    cur=cur*p+offset+(bs[j-1]-b'a') as i64;
                    if set.contains(&cur){
                        f[j]=f[j].max(f[i]+1);
                    }
                }
                if f[n]>1{
                    return true
                }
            }
            false
        };
        words.into_iter().filter(|word| check(word)).collect()
    }
}
