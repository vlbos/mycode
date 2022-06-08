/*
 * @lc app=leetcode id=1032 lang=rust
 *
 * [1032] Stream of Characters
 */

// @lc code=start
use std::collections::HashMap;
#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}
impl TrieNode {
    fn new() -> Self {
        Default::default()
    }
    fn insert(&mut self, word: &String) {
        let mut node = self;
        for c in word.chars().rev() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }
    fn search(&self, word: &String) -> bool {
        let mut node = self;
        for c in word.chars().rev() {
            if node.children.get(&c).is_none() {
                return false;
            }
            node = node.children.get(&c).unwrap();
            if node.is_word {
                return true;
            }
        }
        false
    }
}
struct StreamChecker {
    s: String,
    node: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut node = TrieNode::new();
        for w in &words {
            node.insert(w);
        }
        Self {
            s: String::new(),
            node,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.s.push(letter);
        self.node.search(&self.s)
    }
}

  /**
         * Your StreamChecker object will be instantiated and called as such:
         * let obj = StreamChecker::new(words);
         * let ret_1: bool = obj.query(letter);
         */
// @lc code=end
