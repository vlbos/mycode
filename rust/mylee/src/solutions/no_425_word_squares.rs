// 425\. Word Squares
// ==================

// Given a set of words **(without duplicates)**, find all [word squares](https://en.wikipedia.org/wiki/Word_square) you can build from them.

// A sequence of words forms a valid word square if the _k_th row and column read the exact same string, where 0 ≤ _k_ < max(numRows, numColumns).

// For example, the word sequence `["ball","area","lead","lady"]` forms a word square because each word reads the same both horizontally and vertically.

// b a l l
// a r e a
// l e a d
// l a d y

// **Note:**

// 1.  There are at least 1 and at most 1000 words.
// 2.  All words will have the exact same length.
// 3.  Word length is at least 1 and at most 5.
// 4.  Each word contains only lowercase English alphabet `a-z`.

// **Example 1:**

// **Input:**
// \["area","lead","wall","lady","ball"\]

// **Output:**
// \[
//   \[ "wall",
//     "area",
//     "lead",
//     "lady"
//   \],
//   \[ "ball",
//     "area",
//     "lead",
//     "lady"
//   \]
// \]

// **Explanation:**
// The output consists of two word squares. The order of output does not matter (just the order of words in each word square matters).

// **Example 2:**

// **Input:**
// \["abat","baba","atan","atal"\]

// **Output:**
// \[
//   \[ "baba",
//     "abat",
//     "baba",
//     "atan"
//   \],
//   \[ "baba",
//     "abat",
//     "baba",
//     "atal"
//   \]
// \]

// **Explanation:**
// The output consists of two word squares. The order of output does not matter (just the order of words in each word square matters).

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google) [Oracle](https://leetcode.ca/tags/#Oracle)

// @lc code=start

impl Solution {
    pub fn   word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        // use std::collections::HashMap;
        // let len = words[0].len();
        // let mut cache = HashMap::<&str, Vec<usize>>::new();
        // for (i, w) in words.iter().enumerate() {
        //     for j in 0..len {
        //         cache
        //             .entry(&w[0..j])
        //             .and_modify(|v| v.push(i))
        //             .or_insert(vec![i]);
        //     }
        // }
        // let mut tries: Vec<(usize, String, Vec<usize>)> = vec![(0, String::new(), vec![])];
        // let mut res = vec![];
        // while let Some((i, pre, used)) = tries.pop() {
        //     let ni = i + 1;
        //     for k in &cache[&pre[..]] {
        //         let mut used_copy = used.clone();
        //         used_copy.push(*k);
        //         if ni >= len {
        //             res.push(used_copy);
        //         } else {
        //             let next_pre = String::from_utf8_lossy(
        //                 &used_copy
        //                     .iter()
        //                     .map(|v| words[*v].as_bytes()[ni])
        //                     .collect::<Vec<u8>>(),
        //             )
        //             .to_string();
        //             if cache.contains_key(&next_pre[..]) {
        //                 tries.push((ni, next_pre, used_copy));
        //             }
        //         }
        //     }
        // }
        // res.into_iter()
        //     .map(|v| {
        //         v.into_iter()
        //             .map(|i| &words[i])
        //             .cloned()
        //             .collect::<Vec<_>>()
        //     })
        //     .collect::<Vec<Vec<String>>>()
        if words.is_empty() {
            return Vec::new();
        }
        use std::collections::HashMap;
        let mut prefix_map = HashMap::new();
       pub fn  add_word(
            str: &String,
            i: usize,
            idx: usize,
            prefix_map: &mut HashMap<String, Vec<usize>>,
        ) {
            if i == str.len() {
                return;
            }
            let prefix = str[..i].to_string();
            prefix_map.entry(prefix).or_insert(Vec::new()).push(idx);
            add_word(str, i + 1, idx, prefix_map);
        }
        for (i, w) in words.iter().enumerate() {
            add_word(w, 1, i, &mut prefix_map);
        }
       pub fn  get_square(
            i: usize,
            mut square: Vec<String>,
            words: &Vec<String>,
            prefix_map: &HashMap<String, Vec<usize>>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if i == words[0].len() {
                ans.push(square.clone());
                return;
            }

            let mut prefix = String::new();
            for _ in 0..i {
                prefix.push(square[prefix.len()].as_bytes()[i] as char);
            }
            if let Some(s) = prefix_map.get(&prefix) {
                for &idx in s {
                    square[i] = words[idx].clone();
                    get_square(i + 1, square.clone(), words, prefix_map, ans);
                }
            }
        }
        let mut ans = Vec::new();
        let n = words[0].len();
        for w in &words {
            let square = vec![w.clone(); n];
            get_square(1, square, &words, &prefix_map, &mut ans);
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::{
        assert_equivalent, map_nested_to_string, map_to_string,
    };

    #[test]
   pub fn  test_word_squares_1() {
        let input = map_to_string(&["area", "lead", "wall", "lady", "ball"]);
        let output = map_nested_to_string(&[
            vec!["wall", "area", "lead", "lady"],
            vec!["ball", "area", "lead", "lady"],
        ]);
        assert_equivalent(&Solution::word_squares(input), &output);
    }

    #[test]
   pub fn  test_word_squares_2() {
        let input = map_to_string(&["abat", "baba", "atan", "atal"]);
        let output = map_nested_to_string(&[
            vec!["baba", "abat", "baba", "atan"],
            vec!["baba", "abat", "baba", "atal"],
        ]);
        assert_equivalent(&Solution::word_squares(input), &output);
    }
}
