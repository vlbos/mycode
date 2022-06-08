/*
 * @lc app=leetcode id=745 lang=rust
 *
 * [745] Prefix and Suffix Search
 */

// @lc code=start
use std::collections::HashMap;
struct TrieNode {
    children: HashMap<u8, TrieNode>,
    weight: i32,
}
struct WordFilter {
    trie: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = TrieNode {
            children: HashMap::new(),
            weight: 0,
        };
        let sep = b'a' + 26;
        for (weight, word) in words.iter().enumerate() {
            let mut bw:Vec<u8> = word.bytes().collect();
            bw.push(sep);
            let n = bw.len();
            for i in 0..n {
                let mut cur = &mut trie;
                cur.weight = weight as i32;
                for j in i..2 * n - 1 {
                    let k = bw[j % n];
                    cur = cur.children
                        .entry(k)
                        .and_modify(|x| x.weight = weight as i32)
                        .or_insert(TrieNode {
                            children: HashMap::new(),
                            weight: weight as i32,
                        });
                }
            }
        }
        Self { trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut cur = &self.trie;
        let mut letters :Vec<u8>= suffix.bytes().collect();
        letters.push(b'a' + 26);
        letters.extend(prefix.bytes().collect::<Vec<u8>>());
        for b in &letters {
            if cur.children.get(b).is_none() {
                return -1;
            }
            cur = &cur.children[b];
        }
        cur.weight
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */
// @lc code=end
