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
