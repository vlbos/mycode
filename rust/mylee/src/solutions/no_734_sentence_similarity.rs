// 734\. Sentence Similarity
// =========================

// Given two sentences `words1, words2` (each represented as an array of strings), and a list of similar word pairs `pairs`, determine if two sentences are similar.

// For example, "great acting skills" and "fine drama talent" are similar, if the similar word pairs are `pairs = [["great", "fine"], ["acting","drama"], ["skills","talent"]]`.

// Note that the similarity relation is not transitive. For example, if "great" and "fine" are similar, and "fine" and "good" are similar, "great" and "good" are **not** necessarily similar.

// However, similarity is symmetric. For example, "great" and "fine" being similar is the same as "fine" and "great" being similar.

// Also, a word is always similar with itself. For example, the sentences `words1 = ["great"], words2 = ["great"], pairs = []` are similar, even though there are no specified similar word pairs.

// Finally, sentences can only be similar if they have the same number of words. So a sentence like `words1 = ["great"]` can never be similar to `words2 = ["doubleplus","good"]`.

// **Note:**

// *   The length of `words1` and `words2` will not exceed `1000`.
// *   The length of `pairs` will not exceed `2000`.
// *   The length of each `pairs[i]` will be `2`.
// *   The length of each `words[i]` and `pairs[i][j]` will be in the range `[1, 20]`.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
// use std::collections::HashSet;

impl Solution {
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        // let sps = similar_pairs
        //     .iter()
        //     .map(|sp| (&sp[0] as &str, &sp[1] as &str))
        //     .collect::<HashSet<(&str, &str)>>();
        // let len1 = sentence1.len();
        // let len2 = sentence2.len();
        // if len1 != len2 {
        //     return false;
        // } else {
        //     for i in 0..len1 {
        //         let w1 = &sentence1[i] as &str;
        //         let w2 = &sentence2[i] as &str;
        //         if w1 != w2 {
        //             let p1 = &(w1, w2);
        //             let p2 = &(w2, w1);
        //             if !sps.contains(&p1) && !sps.contains(p2) {
        //                 return false;
        //             }
        //         }
        //     }
        //     true
        // }
        let pairs: std::collections::HashSet<(&str, &str)> = similar_pairs
            .iter()
            .map(|x| (x[0].as_str(), x[1].as_str()))
            .collect();
        if sentence1.len() != sentence2.len() {
            return false;
        }
        for (w1, w2) in sentence1.iter().zip(&sentence2) {
            if w1 == w2 {
                continue;
            }
            if !pairs.contains(&(w1.as_str(), w2.as_str()))
                && !pairs.contains(&(w2.as_str(), w1.as_str()))
            {
                return false;
            }
        }
        true
    }
}
// @lc code=end
 
#[allow(dead_code)] 
 struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lc_matrix_s, lc_vec_s};

    #[test]
    fn test_are_sentences_similar_1() {
        assert_eq!(
            Solution::are_sentences_similar(
                lc_vec_s!["great", "haha"],
                lc_vec_s!["fine", "haha"],
                lc_matrix_s![["great", "fine"], ["drama", "acting"], ["skills", "talent"]]
            ),
            true
        );
    }
}
