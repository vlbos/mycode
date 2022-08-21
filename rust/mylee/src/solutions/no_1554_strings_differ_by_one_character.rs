// 1554\. Strings Differ by One Character
// ======================================

// Given a list of strings `dict` where all the strings are of the same length.

// Return `True` if there are 2 strings that only differ by 1 character in the same index, otherwise return `False`.

// **Follow up:** Could you solve this problem in O(n\*m) where n is the length of `dict` and m is the length of each string.

// **Example 1:**

// **Input:** dict = \["abcd","acbd", "aacd"\]
// **Output:** true
// **Output:** Strings "a**b**cd" and "a**a**cd" differ only by one character in the index 1.

// **Example 2:**

// **Input:** dict = \["ab","cd","yz"\]
// **Output:** false

// **Example 3:**

// **Input:** dict = \["abcd","cccc","abyd","abab"\]
// **Output:** true

// **Constraints:**

// *   Number of characters in `dict <= 10^5`
// *   `dict[i].length == dict[j].length`
// *   `dict[i]` should be unique.
// *   `dict[i]` contains only lowercase English letters.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

// strings_differ_by_one_character

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn differ_by_one(dict: Vec<String>) -> bool {
        if dict.len() < 2 {
            return false;
        }
        let mut s = std::collections::HashSet::new();
        for d in dict {
            let mut dv: Vec<char> = d.chars().collect();
            for i in 0..dv.len() {
                let mut t = dv.clone();
                t[i] = '*';
                if !s.insert(t) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_differ_by_one_1() {
        assert!(Solution::differ_by_one(
            ["abcd", "acbd", "aacd"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_differ_by_one_2() {
        assert!(!Solution::differ_by_one(
            ["ab", "cd", "yz"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_differ_by_one_3() {
        assert!(Solution::differ_by_one(
            ["abcd", "cccc", "abyd", "abab"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
        ));
    }
}
