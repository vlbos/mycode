// 737\. Sentence Similarity II
// ============================

// Given two sentences `words1, words2` (each represented as an array of strings), and a list of similar word pairs `pairs`, determine if two sentences are similar.

// For example, `words1 = ["great", "acting", "skills"]` and `words2 = ["fine", "drama", "talent"]` are similar,
// if the similar word pairs are `pairs = [["great", "good"], ["fine", "good"], ["acting","drama"], ["skills","talent"]]`.

// Note that the similarity relation **is** transitive. For example, if "great" and "good" are similar, and "fine" and "good" are similar, then "great" and "fine" **are similar**.

// Similarity is also symmetric. For example, "great" and "fine" being similar is the same as "fine" and "great" being similar.

// Also, a word is always similar with itself. For example, the sentences `words1 = ["great"], words2 = ["great"], pairs = []` are similar, even though there are no specified similar word pairs.

// Finally, sentences can only be similar if they have the same number of words. So a sentence like `words1 = ["great"]` can never be similar to `words2 = ["doubleplus","good"]`.

// **Note:**

// *   The length of `words1` and `words2` will not exceed `1000`.
// *   The length of `pairs` will not exceed `2000`.
// *   The length of each `pairs[i]` will be `2`.
// *   The length of each `words[i]` and `pairs[i][j]` will be in the range `[1, 20]`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::HashMap;

// pub  struct UnionFind {
//     sz: Vec<usize>,
//     id: Vec<usize>,
//     size: usize,
// }

// impl UnionFind {
//     pub fn   new(size: usize) -> Self {
//         Self {
//             sz: vec![1usize; size],
//             id: (0usize..size).collect(),
//             size,
//         }
//     }

//     pub fn   find(&self, mut p: usize) -> usize {
//         while self.id[p] != p {
//             p = self.id[p];
//         }
//         p
//     }

//     pub fn   connected(&self, p: usize, q: usize) -> bool {
//         self.find(p) == self.find(q)
//     }

//     pub fn   union(&mut self, p: usize, q: usize) {
//         let pid = self.find(p);
//         let qid = self.find(q);
//         if pid == qid {
//             return;
//         }
//         if self.sz[pid] >= self.sz[qid] {
//             self.id[qid] = self.id[pid];
//             self.sz[pid] += self.sz[qid];
//         } else {
//             self.id[pid] = self.id[qid];
//             self.sz[qid] = self.sz[pid];
//         }
//         self.size -= 1;
//     }
// }

impl Solution {
    pub fn are_sentences_similar_two(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        // if words1.len() != words2.len() {
        //     return false;
        // }
        // let mut dict = HashMap::<&str, usize>::new();
        // for p in pairs.iter() {
        //     for i in p {
        //         let size = dict.len();
        //         dict.entry(i as &str).or_insert_with(|| size);
        //     }
        // }
        // let mut uf = UnionFind::new(dict.len());
        // for p in pairs.iter() {
        //     let pid = dict[&p[0] as &str];
        //     let qid = dict[&p[1] as &str];
        //     uf.union(pid, qid);
        // }
        // let len = words2.len();
        // for i in 0..len {
        //     let p = &words1[i] as &str;
        //     let q = &words2[i] as &str;
        //     if p != q {
        //         let pid_opt = dict.get(p);
        //         let qid_opt = dict.get(q);
        //         if let (Some(&pid), Some(&qid)) = (pid_opt, qid_opt) {
        //             if !uf.connected(pid, qid) {
        //                 return false;
        //             }
        //         } else {
        //             return false;
        //         }
        //     }
        // }
        // true
        //  pub fn are_sentences_similar_two(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
        if sentence1.len() != sentence2.len() {
            return false;
        }
        use std::collections::{HashMap, HashSet};
        let mut m = HashMap::new();
        for p in &similar_pairs {
            m.entry(p[0].clone())
                .or_insert(HashSet::new())
                .insert(p[1].clone());
            m.entry(p[1].clone())
                .or_insert(HashSet::new())
                .insert(p[0].clone());
        }
        pub fn dfs(
            m: &HashMap<String, HashSet<String>>,
            w1: &String,
            w2: &String,
            visited: &mut HashSet<String>,
        ) -> bool {
            if w1 == w2 {
                return true;
            }
            visited.insert(w1.clone());
            for w in m.get(w1).unwrap_or(&HashSet::new()) {
                if !visited.contains(w) && dfs(m, w, w2, visited) {
                    return true;
                }
            }
            false
        }
        for (w1, w2) in sentence1.iter().zip(&sentence2) {
            let mut visited = HashSet::new();
            if !dfs(&m, w1, w2, &mut visited) {
                return false;
            }
        }

        true
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lc_matrix_s, lc_vec_s};

    #[test]
    pub fn test_are_sentences_similar_two_1() {
        assert_eq!(
            Solution::are_sentences_similar_two(
                lc_vec_s!["great", "acting", "skills"],
                lc_vec_s!["fine", "drama", "talent"],
                lc_matrix_s![
                    ["great", "good"],
                    ["fine", "good"],
                    ["drama", "acting"],
                    ["skills", "talent"]
                ]
            ),
            true
        );
    }
}
