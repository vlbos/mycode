/*
 * @lc app=leetcode id=211 lang=rust
 *
 * [211] Design Add and Search Words Data Structure
 */

// @lc code=start
#[derive(Default)]
pub struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    has_value: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
         Self::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;

        for c in word.into_bytes() {
            node = node.children[usize::from(c - b'a')].get_or_insert_with(Box::default);
        }

        node.has_value = true;
    }

    fn search_helper(&self, word: &[u8]) -> bool {
        if let Some((&first, rest)) = word.split_first() {
            if let Some(child_slot) = self
                .children
                .get(first.wrapping_sub(b'a') as usize)
                .map(Option::as_deref)
            {
                child_slot.map_or(false, |child| child.search_helper(rest))
            } else {
                for child in self.children.iter().filter_map(Option::as_deref) {
                    if child.search_helper(rest) {
                        return true;
                    }
                }

                false
            }
        } else {
            self.has_value
        }
    }

    fn search(&self, word: String) -> bool {
        self.search_helper(word.as_bytes())
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
// @lc code=end

