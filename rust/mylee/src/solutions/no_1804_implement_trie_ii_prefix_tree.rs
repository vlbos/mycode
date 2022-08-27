// 1804\. Implement Trie II (Prefix Tree)
// ======================================

// A [**trie**](https://en.wikipedia.org/wiki/Trie) (pronounced as "try")
// or **prefix tree** is a tree data structure used to efficiently store
// and retrieve keys in a dataset of strings.
// There are various applications of this data structure, such as autocomplete and spellchecker.

// Implement the Trie class:

// *   `Trie()` Initializes the trie object.
// *   `void insert(String word)` Inserts the string `word` into the trie.
// *   `int countWordsEqualTo(String word)` Returns the number of instances of the string `word` in the trie.
// *   `int countWordsStartingWith(String prefix)` Returns the number of strings in the trie that have the string `prefix` as a prefix.
// *   `void erase(String word)` Erases the string `word` from the trie.

// **Example 1:**

// **Input**
// \["Trie", "insert", "insert", "countWordsEqualTo", "countWordsStartingWith", "erase", "countWordsEqualTo", "countWordsStartingWith", "erase", "countWordsStartingWith"\]
// \[\[\], \["apple"\], \["apple"\], \["apple"\], \["app"\], \["apple"\], \["apple"\], \["app"\], \["apple"\], \["app"\]\]
// **Output**
// \[null, null, null, 2, 2, null, 1, 1, null, 0\]

// **Explanation**
// Trie trie = new Trie();
// trie.insert("apple");               // Inserts "apple".
// trie.insert("apple");               // Inserts another "apple".
// trie.countWordsEqualTo("apple");    // There are two instances of "apple" so return 2.
// trie.countWordsStartingWith("app"); // "app" is a prefix of "apple" so return 2.
// trie.erase("apple");                // Erases one "apple".
// trie.countWordsEqualTo("apple");    // Now there is only one instance of "apple" so return 1.
// trie.countWordsStartingWith("app"); // return 1
// trie.erase("apple");                // Erases "apple". Now the trie is empty.
// trie.countWordsStartingWith("app"); // return 0

// **Constraints:**

// *   `1 <= word.length, prefix.length <= 2000`
// *   `word` and `prefix` consist only of lowercase English letters.
// *   At most `3 * 104` calls **in total** will be made to `insert`, `countWordsEqualTo`, `countWordsStartingWith`, and `erase`.
// *   It is guaranteed that for any function call to `erase`, the string `word` will exist in the trie.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

use std::collections::HashMap;
#[allow(dead_code)]
pub struct Trie {
    children: HashMap<char, Trie>,
    word_count: i32,
    prefix_count: i32,
}
use std::cell::RefCell;
use std::rc::Rc;
impl Trie {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            word_count: 0,
            prefix_count: 0,
        }
    }
    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(Trie::new());
            node.prefix_count += 1;
        }
        node.word_count += 1;
    }
    pub fn count_words_equal_to(&self, word: String) -> i32 {
        let mut node = self;
        for c in word.chars() {
            if let Some(next) = node.children.get(&c) {
                node = next;
            } else {
                return 0;
            }
        }
        node.word_count
    }
    pub fn count_words_starting_with(&self, prefix: String) -> i32 {
        let mut node = self;
        for c in prefix.chars() {
            if let Some(next) = node.children.get(&c) {
                node = next;
            } else {
                return 0;
            }
        }
        node.prefix_count
    }
    pub fn erase(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(Trie::new());
            node.prefix_count -= 1;
        }
        node.word_count -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_trie_1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string()); // Inserts "apple".
        trie.insert("apple".to_string()); // Inserts another "apple".
        assert_eq!(2, trie.count_words_equal_to("apple".to_string())); // There are two instances of "apple" so return 2.
        assert_eq!(2, trie.count_words_starting_with("app".to_string())); // "app" is a prefix of "apple" so return 2.
        trie.erase("apple".to_string()); // Erases one "apple".
        assert_eq!(1, trie.count_words_equal_to("apple".to_string())); // Now there is only one instance of "apple" so return 1.
        assert_eq!(1, trie.count_words_starting_with("app".to_string())); // return 1
        trie.erase("apple".to_string()); // Erases "apple". Now the trie is empty.
        assert_eq!(0, trie.count_words_starting_with("app".to_string())); // return 0
    }
}
