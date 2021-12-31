/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
#[derive(Default)]
struct Trie {
 children: [Option<Box<Trie>>; 26],
    has_value: bool,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
           Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;

        for c in word.into_bytes() {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
        }

        node.has_value = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;

        for c in word.into_bytes() {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                node = child;
            } else {
                return false;
            }
        }

        node.has_value
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;

        for c in prefix.into_bytes() {
            if let Some(child) = node.children[usize::from(c - b'a')].as_deref() {
                node = child;
            } else {
                return false;
            }
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

