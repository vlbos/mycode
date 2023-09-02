/*
 * @lc app=leetcode id=212 lang=rust
 *
 * [212] Word Search II
 */

// @lc code=start
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct TrieNode {
    word: String,
    children: HashMap<char, Option<Rc<RefCell<TrieNode>>>>,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            word: String::new(),
            children: HashMap::new(),
        }
    }
}
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn insert_trie(root: Option<Rc<RefCell<TrieNode>>>, word: &String) {
            let mut node = root;
            for c in word.chars() {
                node.as_mut().unwrap().borrow_mut()
                    .children
                    .entry(c)
                    .or_insert(Some(Rc::new(RefCell::new(TrieNode::new()))));
                let nn = node.as_ref().unwrap().borrow().children.get(&c).unwrap().clone();
                node = nn;
            }
            node.as_mut().unwrap().borrow_mut().word = word.clone();
        }
        fn dfs(
            board: &mut Vec<Vec<char>>,
            i: usize,
            j: usize,
            root: &mut Option<Rc<RefCell<TrieNode>>>,
            ans: &mut HashSet<String>,
        ) {
            let ch = board[i][j];
            if root.is_none() || root.as_ref().unwrap().borrow().children.get(&ch).is_none() {
                return;
            }
            let mut next = root.as_ref().unwrap().borrow().children.get(&ch).unwrap().clone();
            if !next.as_ref().unwrap().borrow().word.is_empty() {
                ans.insert(next.as_ref().unwrap().borrow().word.clone());
                next.as_mut().unwrap().borrow_mut().word = String::new();
            }
            if !next.as_ref().unwrap().borrow().children.is_empty() {
                board[i][j] = '#';
                let dirs = [0, 1, 0, -1, 0];
                let (m, n) = (board.len() as i32, board[0].len() as i32);
                for k in 0..dirs.len() - 1 {
                    let (ni, nj) = (i as i32 + dirs[k], j as i32 + dirs[k + 1]);
                    if ni < 0 || ni >= m || nj < 0 || nj >= n {
                        continue;
                    }
                    let (ii, jj) = (ni as usize, nj as usize);
                    if board[ii][jj] != '#' {
                        dfs(board, ii, jj, &mut next, ans);
                    }
                }
                board[i][j] = ch;
            }
            if next.as_ref().unwrap().borrow().children.is_empty() {
                root.as_mut().unwrap().borrow_mut().children.remove(&ch);
            }
        }
        use std::collections::HashSet;
        let mut root = Some(Rc::new(RefCell::new(TrieNode::new())));
        let mut ans = HashSet::new();
        for word in &words {
            insert_trie(root.clone(), word);
        }
        let (m, n) = (board.len(), board[0].len());
        let mut board = board;
        for i in 0..m {
            for j in 0..n {
                dfs(&mut board, i, j, &mut root, &mut ans);
            }
        }
        ans.iter().cloned().collect::<Vec<String>>()
    }
}
// @lc code=end
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie=Trie::new();
        for w in &words{
            trie.insert(w);
        }
        fn dfs(mut node:&Trie,i:usize,j:usize,board: &mut Vec<Vec<char>>,ans:&mut HashSet<String>){
             let ch=board[i][j];
            if let Some(child)=node.children.get(&ch){
                node=child;
            }else{
                return
            }
           if !node.word.is_empty(){
               ans.insert(node.word.clone());
           }
           board[i][j]='#';
           let (m,n)=(board.len() as i32,board[0].len() as i32);
           for d in [0,1,0,-1,0].windows(2){
               let (ni,nj)=(i as i32+d[0],j as i32+d[1]);
               if ni>=0 && ni<m &&  nj>=0 && nj<n{
                   dfs(node,ni as usize,nj as usize,board,ans);
               }
           }
           board[i][j]=ch;
        }
        let (m,n)=(board.len(),board[0].len());
        let mut ans=HashSet::new();
        for i in 0..m{
            for j in 0..n{
                dfs(&trie,i,j,&mut board,&mut ans);
            }
        }
        ans.into_iter().collect()
    }
}
use std::collections::{HashMap,HashSet};
struct Trie{
    children:HashMap<char,Trie>,
    word:String,
}
impl Trie{
    fn new()->Self{
        Self{children:HashMap::new(),word:String::new()}
    }
    fn insert(&mut self,word:&String){
        let mut node=self;
        for c in word.chars(){
            node=node.children.entry(c).or_insert(Trie::new());
        }
        node.word=word.clone();
    }
}