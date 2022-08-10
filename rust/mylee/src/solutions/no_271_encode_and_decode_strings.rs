// 271\. Encode and Decode Strings
// ===============================

// Design an algorithm to encode **a list of strings** to **a string**.
// The encoded string is then sent over the network and is decoded back to the original list of strings.

// Machine 1 (sender) has the function:

// string encode(vector<string> strs) {
//   // ... your code
//   return encoded\_string;
// }

// Machine 2 (receiver) has the function:

// vector<string> decode(string s) {
//   //... your code
//   return strs;
// }

// So Machine 1 does:

// string encoded\_string = encode(strs);

// and Machine 2 does:

// vector<string> strs2 = decode(encoded\_string);

// `strs2` in Machine 2 should be the same as `strs` in Machine 1.

// Implement the `encode` and `decode` methods.

// **Note:**

// *   The string may contain any possible characters out of 256 valid ascii characters.
// Your algorithm should be generalized enough to work on any possible characters.
// *   Do not use class member/global/static variables to store states. Your encode and decode algorithms should be stateless.
// *   Do not rely on any library method such as `eval` or serialize methods. You should implement your own encode/decode algorithm.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Square](https://leetcode.ca/tags/#Square) [Twitter](https://leetcode.ca/tags/#Twitter)
// @lc code=start
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::iter::FromIterator;
// use std::mem::swap;
// use std::rc::Rc;

// #[derive(Debug, Clone)]
// struct TrieNode {
//     id: usize,
//     content: Vec<char>,
//     children: HashMap<char, usize>,
//     term_ids: Vec<usize>,
// }

// const SP1: &'static str = "临";
// const SP2: &'static str = "兵";
// const SP3: &'static str = "斗";
// const SP4: &'static str = "者";

// impl TrieNode {
//     pub fn serialize(&self) -> String {
//         format!(
//             "{}{}{}{}{}{}{}",
//             self.id,
//             SP2,
//             self.content.iter().cloned().collect::<String>(),
//             SP2,
//             self.children
//                 .iter()
//                 .map(|(k, v)| format!("{}{}{}", k, SP4, v))
//                 .collect::<Vec<String>>()
//                 .join(SP3),
//             SP2,
//             self.term_ids
//                 .iter()
//                 .map(|i| i.to_string())
//                 .collect::<Vec<String>>()
//                 .join(SP3)
//         )
//     }

//     pub fn deserialize(s: &str) -> TrieNode {
//         let parts = s.split(SP2).collect::<Vec<_>>();
//         let id = parts[0].parse::<usize>().unwrap();
//         let content = parts[1].chars().collect::<Vec<char>>();
//         let children = HashMap::<char, usize>::from_iter(
//             parts[2].split(SP3).filter(|kv| !kv.is_empty()).map(|kv| {
//                 let k_v_arr = kv.split(SP4).collect::<Vec<_>>();
//                 (
//                     k_v_arr[0].chars().next().unwrap(),
//                     k_v_arr[1].parse::<usize>().unwrap(),
//                 )
//             }),
//         );
//         let term_ids = parts[3]
//             .split(SP3)
//             .filter(|s| !s.is_empty())
//             .map(|i| i.parse::<usize>().unwrap())
//             .collect::<Vec<usize>>();
//         TrieNode {
//             id,
//             content,
//             children,
//             term_ids,
//         }
//     }
// }

// struct TrieTree {
//     root: Rc<RefCell<TrieNode>>,
//     nodes: Vec<Rc<RefCell<TrieNode>>>,
// }

// impl TrieTree {
//     pub fn new() -> Self {
//         let root = Rc::new(RefCell::new(TrieNode {
//             id: 0,
//             content: vec![],
//             children: HashMap::new(),
//             term_ids: vec![],
//         }));
//         TrieTree {
//             root: root.clone(),
//             nodes: vec![root],
//         }
//     }

//     pub fn gen(&mut self, content: Vec<char>) -> Rc<RefCell<TrieNode>> {
//         let node = Rc::new(RefCell::new(TrieNode {
//             id: self.nodes.len(),
//             content,
//             children: HashMap::new(),
//             term_ids: vec![],
//         }));
//         self.nodes.push(node.clone());
//         node
//     }

//     pub fn add(&mut self, content: &[char], term_id: usize) {
//         self.insert(self.root.clone(), content, term_id);
//     }

//     pub fn insert(&mut self, node_rc: Rc<RefCell<TrieNode>>, content: &[char], term_id: usize) {
//         let mut i = 0;
//         let mut node = node_rc.borrow_mut();
//         while i < content.len() && i < node.content.len() {
//             if content[i] != node.content[i] {
//                 break;
//             }
//             i += 1;
//         }
//         if i < node.content.len() {
//             let new_key: char = node.content[i];
//             let new_content = node.content.drain(i..).collect::<Vec<_>>();
//             let new_node_rc = self.gen(new_content);
//             let mut new_node = new_node_rc.borrow_mut();
//             swap(&mut new_node.term_ids, &mut node.term_ids);
//             swap(&mut new_node.children, &mut node.children);
//             node.children.insert(new_key, new_node.id);
//         }
//         if i < content.len() {
//             let key: char = content[i];
//             node.children
//                 .entry(key)
//                 .and_modify(|c| {
//                     self.insert(self.nodes[*c].clone(), &content[i..], term_id);
//                 })
//                 .or_insert_with(|| {
//                     let new_node_rc = self.gen(content[i..].to_vec());
//                     let mut new_node = new_node_rc.borrow_mut();
//                     new_node.term_ids.push(term_id);
//                     new_node.id
//                 });
//         } else {
//             node.term_ids.push(term_id);
//         }
//     }

//     pub fn serialize(&self) -> String {
//         self.nodes
//             .iter()
//             .map(|n| {
//                 let node_b = n.borrow();
//                 node_b.serialize()
//             })
//             .collect::<Vec<String>>()
//             .join(SP1)
//     }

//     pub fn deserialize(inputs: &str) -> TrieTree {
//         let nodes = inputs
//             .split(SP1)
//             .map(|s| Rc::new(RefCell::new(TrieNode::deserialize(s))))
//             .collect::<Vec<_>>();
//         TrieTree {
//             root: nodes[0].clone(),
//             nodes,
//         }
//     }

//     pub fn retrieve(&self) -> Vec<String> {
//         let res_rc = Rc::new(RefCell::new(vec![]));
//         {
//             self.retrieve_rec(self.nodes[0].clone(), String::from(""), res_rc.clone());
//         }
//         let mut res = res_rc.borrow_mut();
//         res.sort_by_key(|&(_, id)| id);
//         res.iter().map(|e| e.0.clone()).collect::<Vec<String>>()
//     }

//     pub fn retrieve_rec(
//         &self,
//         node_rc: Rc<RefCell<TrieNode>>,
//         mut visited: String,
//         res_rc: Rc<RefCell<Vec<(String, usize)>>>,
//     ) {
//         let node = node_rc.borrow();
//         visited.extend(node.content.iter());
//         {
//             let mut res = res_rc.borrow_mut();
//             for i in &node.term_ids {
//                 res.push((visited.clone(), *i));
//             }
//         }
//         for c in node.children.values() {
//             let c_node = self.nodes[*c].clone();
//             self.retrieve_rec(c_node, visited.clone(), res_rc.clone());
//         }
//     }
// }

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        // let mut trie = TrieTree::new();
        // for (i, s) in strs.into_iter().enumerate() {
        //     let chars = s.chars().collect::<Vec<_>>();
        //     trie.add(&chars, i);
        // }
        // trie.serialize()
        let strs: Vec<String> = strs
            .into_iter()
            .map(|mut x| {
                x = x.replace("|", "||");
                x
            })
            .collect();
        strs.join("|")
    }

    fn decode(&self, s: String) -> Vec<String> {
        // let trie = TrieTree::deserialize(&s);
        // trie.retrieve()
        let mut ans = Vec::new();
        let mut j = 0;
        let mut k = 0;
        let mut indices = Vec::new();
        for (i, _) in s.match_indices("|") {
            if i == j + 1 {
                k += 1;
            } else {
                if k % 2 == 0 {
                    indices.push(i);
                }
                k = 0;
            }
            j = i;
        }
        j = 0;
        for &i in &indices {
            let ss = s[j..i].replace("||", "|");
            ans.push(ss.to_string());
            j = i + 1;
        }
        if j < s.len() {
            let ss = s[j..].replace("||", "|");
            ans.push(ss.to_string());
        }
        ans
    }
}
// @lc code=end 
#[allow(dead_code)] 
 struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;

    #[test]
    fn test_encode_decode() {
        let inputs = map_to_string(&["abc", "acd", "abcde", "efg"]);
        let codec = Codec {};
        let message = codec.encode(inputs.clone());
        println!("{}", message);
        let outputs = codec.decode(message);
        assert_eq!(inputs, outputs);
    }
}
