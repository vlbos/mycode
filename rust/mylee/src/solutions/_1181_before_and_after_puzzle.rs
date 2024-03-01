// 1181\. Before and After Puzzle
// ==============================

// Given a list of `phrases`, generate a list of Before and After puzzles.

// A _phrase_ is a string that consists of lowercase English letters and spaces only. No space appears in the start or the end of a phrase.
// There are no consecutive spaces in a phrase.

// _Before and After puzzles_ are phrases that are formed by merging two phrases where the **last word of the first phrase** is the same as the **first word of the second phrase**.

// Return the Before and After puzzles that can be formed by every two phrases `phrases[i]` and `phrases[j]` where `i != j`.
// Note that the order of matching two phrases matters, we want to consider both orders.

// You should return a list of **distinct** strings **sorted lexicographically**.

// **Example 1:**

// **Input:** phrases = \["writing code","code rocks"\]
// **Output:** \["writing code rocks"\]

// **Example 2:**

// **Input:** phrases = \["mission statement",
//                   "a quick bite to eat",
//                   "a chip off the old block",
//                   "chocolate bar",
//                   "mission impossible",
//                   "a man on a mission",
//                   "block party",
//                   "eat my words",
//                   "bar of soap"\]
// **Output:** \["a chip off the old block party",
//          "a man on a mission impossible",
//          "a man on a mission statement",
//          "a quick bite to eat my words",
//          "chocolate bar of soap"\]

// **Example 3:**

// **Input:** phrases = \["a","b","a"\]
// **Output:** \["a"\]

// **Constraints:**

// *   `1 <= phrases.length <= 100`
// *   `1 <= phrases[i].length <= 100`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [clutter](https://leetcode.ca/tags/#clutter)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn before_and_after_puzzles(phrases: Vec<String>) -> Vec<String> {
        use std::collections::{HashMap, HashSet};
        let (mut first, mut last) = (
            HashMap::<String, Vec<usize>>::new(),
            HashMap::<String, Vec<usize>>::new(),
        );
        let mut ans = HashSet::new();
        for (i, p) in phrases.iter().enumerate() {
            last.entry(p.split_ascii_whitespace().last().unwrap().to_string())
                .or_insert(Vec::new())
                .push(i);
            first
                .entry(p.split_ascii_whitespace().next().unwrap().to_string())
                .or_insert(Vec::new())
                .push(i);
        }
        for w in first
            .keys()
            .cloned()
            .collect::<HashSet<String>>()
            .intersection(&last.keys().cloned().collect::<HashSet<String>>())
        {
            for &i in first.get(w).unwrap() {
                for &j in last.get(w).unwrap() {
                    if i != j {
                        ans.insert(phrases[j].clone() + &phrases[i][w.len()..]);
                    }
                }
            }
        }
        let mut ans: Vec<String> = ans.into_iter().collect();
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_before_and_after_puzzles_1() {
        assert_eq!(
            vec!["writing code rocks".to_string()],
            Solution::before_and_after_puzzles(
                ["writing code", "code rocks"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_before_and_after_puzzles_2() {
        assert_eq!(
            [
                "a chip off the old block party",
                "a man on a mission impossible",
                "a man on a mission statement",
                "a quick bite to eat my words",
                "chocolate bar of soap"
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
            Solution::before_and_after_puzzles(
                [
                    "mission statement",
                    "a quick bite to eat",
                    "a chip off the old block",
                    "chocolate bar",
                    "mission impossible",
                    "a man on a mission",
                    "block party",
                    "eat my words",
                    "bar of soap"
                ]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_before_and_after_puzzles_3() {
        assert_eq!(
            vec!["a".to_string()],
            Solution::before_and_after_puzzles(
                ["a", "b", "a"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
}
