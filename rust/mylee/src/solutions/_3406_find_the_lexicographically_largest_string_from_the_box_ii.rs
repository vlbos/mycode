// [3406\. Find the Lexicographically Largest String From the Box II 🔒](https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-ii)
// ==============================================================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// You are given a string `word`, and an integer `numFriends`.

// Alice is organizing a game for her `numFriends` friends. There are multiple rounds in the game, where in each round:

// *   `word` is split into `numFriends` **non-empty** strings, such that no previous round has had the **exact** same split.
// *   All the split words are put into a box.

// Find the **lexicographically largest** string from the box after all the rounds are finished.

// A string `a` is **lexicographically smaller** than a string `b` if in the first position where `a` and `b` differ, string `a` has a letter that appears earlier in the alphabet than the corresponding letter in `b`.
// If the first `min(a.length, b.length)` characters do not differ, then the shorter string is the lexicographically smaller one.

// **Example 1:**

// **Input:** word = "dbca", numFriends = 2

// **Output:** "dbc"

// **Explanation:**

// All possible splits are:

// *   `"d"` and `"bca"`.
// *   `"db"` and `"ca"`.
// *   `"dbc"` and `"a"`.

// **Example 2:**

// **Input:** word = "gggg", numFriends = 4

// **Output:** "g"

// **Explanation:**

// The only possible split is: `"g"`, `"g"`, `"g"`, and `"g"`.

// **Constraints:**

// *   `1 <= word.length <= 2 * 105`
// *   `word` consists only of lowercase English letters.
// *   `1 <= numFriends <= word.length`

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        word
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_answer_string_1() {
        assert_eq!(
            String::from("dbc"),
            Solution::answer_string(String::from("dbca"), 2)
        );
    }
    #[test]
    pub fn test_answer_string_2() {
        assert_eq!(
            String::from("g"),
            Solution::answer_string(String::from("gggg"), 4)
        );
    }
}

// Time:  O(n)
// Space: O(1)

// greedy
// class Solution {
// public:
//     string answer_string(string word, int numFriends) {
//         if (numFriends == 1) {
//             return word;
//         }
//         int idx = 0;
//         for (int i = 1, l = 0; i < size(word); ++i) {
//             if (word[i] == word[idx + l]) {
//                 ++l;
//             } else if (word[i] < word[idx + l]) {
//                 l = 0;
//             } else if (word[i] > word[idx + l]) {
//                 if (word[i - l] >= word[i]) {
//                     idx = i - l;
//                 } else {
//                     idx = i;
//                 }
//                 l = 0;
//             }
//         }
//         return word.substr(idx, (size(word) - max((numFriends - 1) - idx, 0)) - idx);
//     }
// };
