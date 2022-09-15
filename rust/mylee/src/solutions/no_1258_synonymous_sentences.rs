// 1258\. Synonymous Sentences
// ===========================

// Given a list of pairs of equivalent words `synonyms` and a sentence `text`, Return all possible synonymous sentences **sorted lexicographically**.

// **Example 1:**

// **Input:** synonyms = \[\["happy","joy"\],\["sad","sorrow"\],\["joy","cheerful"\]\],
// text = "I am happy today but was sad yesterday"
// **Output:** \["I am cheerful today but was sad yesterday",
// ​​​​​​"I am cheerful today but was sorrow yesterday",
// "I am happy today but was sad yesterday",
// "I am happy today but was sorrow yesterday",
// "I am joy today but was sad yesterday",
// "I am joy today but was sorrow yesterday"\]

// **Constraints:**

// *   `0 <= synonyms.length <= 10`
// *   `synonyms[i].length == 2`
// *   `synonyms[0] != synonyms[1]`
// *   All words consist of at most `10` English letters only.
// *   `text` is a single space separated sentence of at most `10` words.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Cruise Automation](https://leetcode.ca/tags/#Cruise%20Automation)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn generate_sentences(synonyms: Vec<Vec<String>>, text: String) -> Vec<String> {
        let text: Vec<&str> = text.split_ascii_whitespace().collect();
        let mut q = std::collections::VecDeque::from([text.clone()]);
        for s in &synonyms {
            let mut i = 0;
            while i < q.len() {
                let text = q[i].clone();
                if let Some(j) = text.iter().position(|x| x == &s[0]) {
                    let mut new_text = text.clone();
                    new_text[j] = s[1].as_str();
                    if new_text != text && !q.contains(&new_text) {
                        q.push_back(new_text.clone());
                    }
                }
                if let Some(j) = text.iter().position(|x| x == &s[1]) {
                    let mut new_text = text.clone();
                    new_text[j] = s[0].as_str();
                    if new_text != text && !q.contains(&new_text) {
                        q.push_back(new_text.clone());
                    }
                }
                i += 1;
            }
        }
        let mut ans: Vec<String> = q.iter().map(|x| x.join(" ")).collect();
        ans.sort();
        ans
    }
}

//         use std::collections::{HashMap,BTreeSet};

// impl Solution {
//     pub fn generate_sentences(synonyms: Vec<Vec<String>>, text: String) -> Vec<String> {
// let mut bts = BTreeSet::new();

//         for s in &synonyms {
//             bts.insert(s[0].to_string());
//             bts.insert(s[1].to_string());
//         }

//         for s in text.split_whitespace() {
//             bts.insert(s.to_string());
//         }

//         let n = bts.len();
//         let mut uf = UnionFind::new(n);
//         let mut words = vec![];
//         let mut ids = HashMap::new();

//         for (i, s) in bts.iter().enumerate() {
//             words.push(s.to_string());
//             ids.insert(s.to_string(), i);
//         }

//         for s in synonyms {
//             uf.union(ids[&s[0]], ids[&s[1]]);
//         }

//         let all_groups = uf.groups();
//         let mut groups = vec![];

//         for s in text.split_whitespace() {
//             let wid = ids[s];
//             let gid = uf.find(wid);
//             let group = all_groups[&gid].to_vec();
//             groups.push(group);
//         }

//         let m = groups.len();
//         let mut ans = vec![];
//         fn dfs(groups: &Vec<Vec<usize>>, words: &Vec<String>, selected: &mut Vec<usize>, ans: &mut Vec<String>, begin: usize) {
//         if begin == groups.len() {
//             let s = selected.iter().map(|&i| words[i].to_string()).collect::<Vec<String>>();
//             ans.push(s.join(" "));
//         } else {
//             for &i in &groups[begin] {
//                 selected.push(i);
//                 dfs(groups, words, selected, ans, begin + 1);
//                 selected.pop();
//             }
//         }
//     }
//         dfs(&groups, &words, &mut Vec::new(), &mut ans, 0);

//         ans
//     }

// }

// struct UnionFind {
//     parent: Vec<usize>,
//     n: usize,
// }

// impl UnionFind {
//     fn new(n: usize) -> Self {
//         Self {
//             parent: (0..n).collect(),
//             n,
//         }
//     }

//     fn find(&mut self, i: usize) -> usize {
//         if i != self.parent[i] {
//             self.parent[i] = self.find(self.parent[i]);
//         }

//         self.parent[i]
//     }

//     fn union(&mut self, i: usize, j: usize) {
//         let x = self.find(i);
//         let y = self.find(j);

//         if x != y {
//             self.parent[x] = y;
//         }
//     }

//     fn groups(&mut self) -> HashMap<usize, Vec<usize>> {
//         let mut mp = HashMap::new();

//         for i in 0..self.n {
//             let j = self.find(i);
//             mp.entry(j).or_insert(vec![]).push(i);
//         }

//         mp
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    // [["a","b"],["b","c"],["d","e"],["c","d"]]
    // "a b"
    // 输出：
    // ["a a","a b","a c","a d","b a","b b","b c","b d","c a","c b","c c","c d","d a","d b","d c","d d"]
    // 预期结果：
    // ["a a","a b","a c","a d","a e","b a","b b","b c","b d","b e","c a","c b","c c","c d","c e","d a","d b","d c","d d","d e","e a","e b","e c","e d","e e"]
    #[test]
    pub fn test_generate_sentences_1() {
        assert_eq!(
            [
                "I am cheerful today but was sad yesterday",
                "I am cheerful today but was sorrow yesterday",
                "I am happy today but was sad yesterday",
                "I am happy today but was sorrow yesterday",
                "I am joy today but was sad yesterday",
                "I am joy today but was sorrow yesterday"
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
            Solution::generate_sentences(
                vec![
                    vec!["happy".to_string(), "joy".to_string()],
                    vec!["sad".to_string(), "sorrow".to_string()],
                    vec!["joy".to_string(), "cheerful".to_string()]
                ],
                String::from("I am happy today but was sad yesterday")
            )
        );
    }
}
