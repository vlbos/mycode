// 642\. Design Search Autocomplete System
// =======================================

// Design a search autocomplete system for a search engine. Users may input a sentence (at least one word and end with a special character `'#'`).
// For **each character** they type **except '#'**, you need to return the **top 3** historical hot sentences that have prefix the same as the part of sentence already typed.
// Here are the specific rules:

// 1.  The hot degree for a sentence is defined as the number of times a user typed the exactly same sentence before.
// 2.  The returned top 3 hot sentences should be sorted by hot degree (The first is the hottest one). If several sentences have the same degree of hot,
// you need to use ASCII-code order (smaller one appears first).
// 3.  If less than 3 hot sentences exist, then just return as many as you can.
// 4.  When the input is a special character, it means the sentence ends, and in this case, you need to return an empty list.

// Your job is to implement the following functions:

// The constructor function:

// `AutocompleteSystem(String[] sentences, int[] times):` This is the constructor. The input is **historical data**.
//  `Sentences` is a string array consists of previously typed sentences.
// `Times` is the corresponding times a sentence has been typed. Your system should record these historical data.

// Now, the user wants to input a new sentence. The following function will provide the next character the user types:

// `List<String> input(char c):` The input `c` is the next character typed by the user. The character will only be lower-case letters (`'a'` to `'z'`),
// blank space (`' '`) or a special character (`'#'`). Also, the previously typed sentence should be recorded in your system.
// The output will be the **top 3** historical hot sentences that have prefix the same as the part of sentence already typed.

//

// **Example:**
// **Operation:** AutocompleteSystem(\["i love you", "island","ironman", "i love leetcode"\], \[5,3,2,2\])
// The system have already tracked down the following sentences and their corresponding times:
// `"i love you"` : `5` times
// `"island"` : `3` times
// `"ironman"` : `2` times
// `"i love leetcode"` : `2` times
// Now, the user begins another search:

// **Operation:** input('i')
// **Output:** \["i love you", "island","i love leetcode"\]
// **Explanation:**
// There are four sentences that have prefix `"i"`. Among them, "ironman" and "i love leetcode" have same hot degree. Since `' '` has ASCII code 32 and `'r'` has ASCII code 114, "i love leetcode" should be in front of "ironman". Also we only need to output top 3 hot sentences, so "ironman" will be ignored.

// **Operation:** input(' ')
// **Output:** \["i love you","i love leetcode"\]
// **Explanation:**
// There are only two sentences that have prefix `"i "`.

// **Operation:** input('a')
// **Output:** \[\]
// **Explanation:**
// There are no sentences that have prefix `"i a"`.

// **Operation:** input('#')
// **Output:** \[\]
// **Explanation:**
// The user finished the input, the sentence `"i a"` should be saved as a historical sentence in system. And the following input will be counted as a new search.

//

// **Note:**

// 1.  The input sentence will always start with a letter and end with '#', and only one blank space will exist between two words.
// 2.  The number of **complete sentences** that to be searched won't exceed 100. The length of each sentence including those in the historical data won't exceed 100.
// 3.  Please use double-quote instead of single-quote when you write test cases even for a character input.
// 4.  Please remember to **RESET** your class variables declared in class AutocompleteSystem, as static/class variables are **persisted across multiple test cases**.
//  Please see [here](https://leetcode.com/faq/#different-output) for more details.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Dropbox](https://leetcode.ca/tags/#Dropbox) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Lyft](https://leetcode.ca/tags/#Lyft) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Uber](https://leetcode.ca/tags/#Uber)
// @lc code=start
// use std::cell::RefCell;
// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap};
// use std::ops::{Deref, DerefMut};
// use std::rc::Rc;

// #[derive(Clone, Debug)]
// pub  struct TrieNode {
//     children: HashMap<char, Trie>,
//     terms: usize,
// }

// #[derive(Clone, Debug)]
// pub  struct Trie(Rc<RefCell<TrieNode>>);

// impl Deref for Trie {
//     type Target = Rc<RefCell<TrieNode>>;

//     fn deref(self: &'_ Self) -> &'_ Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for Trie {
//     fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
//         &mut self.0
//     }
// }

// impl Trie {
//     pub fn new() -> Self {
//         Self(Rc::new(RefCell::new(TrieNode {
//             children: HashMap::new(),
//             terms: 0,
//         })))
//     }

//     pub fn add(&mut self, word: &[char], times: usize) {
//         let mut node = self.borrow_mut();
//         if word.is_empty() {
//             node.terms += times;
//         } else {
//             let ch = word[0];
//             if let Some(child_node) = node.children.get_mut(&ch) {
//                 child_node.add(&word[1..], times);
//             } else {
//                 let mut new_node = Trie::new();
//                 new_node.add(&word[1..], times);
//                 node.children.insert(ch, new_node);
//             }
//         }
//     }

//     pub fn match_next(&self, ch: char) -> Option<Trie> {
//         let node = self.borrow();
//         if let Some(child_node) = node.children.get(&ch) {
//             Some(child_node.clone())
//         } else {
//             None
//         }
//     }
// }

// impl IntoIterator for Trie {
//     type Item = (Trie, Vec<char>);
//     type IntoIter = TrieIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         TrieIntoIterator {
//             queue: vec![(self, vec![])],
//         }
//     }
// }

// pub  struct TrieIntoIterator {
//     queue: Vec<(Trie, Vec<char>)>,
// }

// impl Iterator for TrieIntoIterator {
//     type Item = (Trie, Vec<char>);

//     fn next(&mut self) -> Option<(Trie, Vec<char>)> {
//         while let Some((trie, s)) = self.queue.pop() {
//             let is_term = {
//                 let node = trie.borrow_mut();
//                 for (k, c) in &node.children {
//                     let mut ns = s.clone();
//                     ns.push(*k);
//                     self.queue.push((c.clone(), ns));
//                 }
//                 node.terms > 0
//             };
//             if is_term {
//                 return Some((trie, s));
//             }
//         }
//         None
//     }
// }

use std::collections::HashMap;
pub struct Trie {
    times: i32,
    branches: HashMap<char, Option<Box<Trie>>>,
}
impl Trie {
    fn new() -> Self {
        Self {
            times: 0,
            branches: HashMap::new(),
        }
    }
    fn insert(&mut self, s: &String, times: i32) {
        let mut t = self;
        for c in s.chars() {
            t = t
                .branches
                .entry(c)
                .or_insert(Some(Box::new(Trie::new())))
                .as_mut()
                .unwrap();
        }
        t.times += times;
    }
    fn lookup(&self, s: &String) -> Vec<(String, i32)> {
        let mut list = Vec::new();
        let mut t = self;
        for c in s.chars() {
            if let Some(nt) = t.branches.get(&c) {
                t = nt.as_ref().unwrap();
            } else {
                return Vec::new();
            }
        }
        self.traverse(s, t, &mut list);
        list
    }
    fn traverse(&self, s: &String, t: &Trie, list: &mut Vec<(String, i32)>) {
        if t.times > 0 {
            list.push((s.clone(), t.times));
        }
        for c in 'a'..='z' {
            if let Some(nt) = t.branches.get(&c) {
                self.traverse(
                    &(s.to_owned() + c.to_string().as_str()),
                    nt.as_ref().unwrap(),
                    list,
                );
            }
        }
        if let Some(nt) = t.branches.get(&' ') {
            self.traverse(&(s.to_owned() + " "), nt.as_ref().unwrap(), list);
        }
    }
}
pub struct AutocompleteSystem {
    // root: Trie,
    // sentence: Vec<char>,
    // curr: Option<Trie>,
    // to_show: i32,
    root: Trie,
    cur_sent: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AutocompleteSystem {
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        // let sentence = vec![];
        // let root = Trie::new();
        // let mut acs = Self {
        //     root: root.clone(),
        //     sentence,
        //     curr: Some(root),
        //     to_show: 0,
        // };
        // let len = sentences.len();
        // for i in 0..len {
        //     let chars = sentences[i].chars().collect::<Vec<_>>();
        //     acs.root.add(&chars, times[i] as usize);
        // }
        // acs
        let mut root = Trie::new();
        for (i, s) in sentences.iter().enumerate() {
            root.insert(s, times[i]);
        }
        Self {
            root,
            cur_sent: String::new(),
        }
    }

    fn input(&mut self, c: char) -> Vec<String> {
        // if c == '#' {
        //     self.root.add(&self.sentence, 1);
        //     self.sentence.clear();
        //     self.curr = Some(self.root.clone());
        //     vec![]
        // } else {
        //     let mut res = vec![];
        //     self.sentence.push(c);
        //     let new_curr = if let Some(curr_trie) = self.curr.clone() {
        //         curr_trie.match_next(c)
        //     } else {
        //         None
        //     };
        //     if let Some(curr_trie) = new_curr.clone() {
        //         let mut heap = BinaryHeap::<(Reverse<usize>, Vec<char>)>::new();
        //         for (t, suffix) in curr_trie {
        //             let n = t.borrow();
        //             let mut prefix = self.sentence.clone();
        //             prefix.extend_from_slice(&suffix[..]);
        //             let new = (Reverse(n.terms), prefix);
        //             if heap.len() < 3 {
        //                 heap.push(new);
        //             } else {
        //                 let old = heap.pop().unwrap();
        //                 heap.push(if new < old { new } else { old });
        //             }
        //         }
        //         res = heap
        //             .into_sorted_vec()
        //             .into_iter()
        //             .map(|(_, v)| v.into_iter().collect::<String>())
        //             .collect::<Vec<_>>();
        //     }
        //     self.curr = new_curr;
        //     res
        // }
        let mut ans = Vec::new();
        if c == '#' {
            self.root.insert(&self.cur_sent, 1);
            self.cur_sent = String::new();
        } else {
            self.cur_sent.push(c);
            let mut list = self.root.lookup(&self.cur_sent);
            list.sort_by(|a, b| {
                if a.1 == b.1 {
                    a.0.cmp(&b.0)
                } else {
                    b.1.cmp(&a.1)
                }
            });
            ans = (if list.len() > 3 {
                &list[..3]
            } else {
                &list[..]
            })
            .iter()
            .map(|x| x.0.clone())
            .collect();
        }
        ans
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    fn test_autocomplete_system_1() {
        let mut acs = AutocompleteSystem::new(
            lc_vec_s!["i love you", "island", "iroman", "i love leetcode"],
            vec![5, 3, 2, 2],
        );
        assert_eq!(
            acs.input('i'),
            lc_vec_s!["i love you", "island", "i love leetcode"]
        );
        assert_eq!(acs.input(' '), lc_vec_s!["i love you", "i love leetcode"]);
        assert_eq!(acs.input('a'), Vec::<String>::new());
        assert_eq!(acs.input('#'), Vec::<String>::new());
        assert_eq!(
            acs.input('i'),
            lc_vec_s!["i love you", "island", "i love leetcode"]
        );
        assert_eq!(
            acs.input(' '),
            lc_vec_s!["i love you", "i love leetcode", "i a"]
        );
        assert_eq!(acs.input('a'), lc_vec_s!["i a"]);
        assert_eq!(acs.input('#'), Vec::<String>::new());
        assert_eq!(acs.input('i'), lc_vec_s!["i love you", "island", "i a"]);
        assert_eq!(
            acs.input(' '),
            lc_vec_s!["i love you", "i a", "i love leetcode"]
        );
        assert_eq!(acs.input('a'), lc_vec_s!["i a"]);
        assert_eq!(acs.input('#'), Vec::<String>::new());
    }

    #[test]
    fn test_autocomplete_system_2() {
        let mut acs = AutocompleteSystem::new(lc_vec_s!["abc", "abbc", "a"], vec![3, 3, 3]);
        assert_eq!(acs.input('b'), Vec::<String>::new());
        assert_eq!(acs.input('c'), Vec::<String>::new());
        assert_eq!(acs.input('#'), Vec::<String>::new());
        assert_eq!(acs.input('b'), lc_vec_s!["bc"]);
    }
}
