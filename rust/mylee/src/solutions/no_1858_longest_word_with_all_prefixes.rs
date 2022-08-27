// 1858\. Longest Word With All Prefixes[](https://leetcode.ca/2021-07-14-1858-Longest-Word-With-All-Prefixes/#1858-longest-word-with-all-prefixes)
// ================================================================================================================================================

// Level[](https://leetcode.ca/2021-07-14-1858-Longest-Word-With-All-Prefixes/#level)
// ----------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-14-1858-Longest-Word-With-All-Prefixes/#description)
// ----------------------------------------------------------------------------------------------

// Given an array of strings `words`, find the **longest** string in `words` such that **every prefix** of it is also in `words`.

// *   For example, let `words = ["a", "app", "ap"]`. The string `"app"` has prefixes `"ap"` and `"a"`, all of which are in words.

// Return _the string described above. If there is more than one string with the same length, return the **lexicographically smallest** one, and if no string exists, return `""`_.

// **Example 1:**

// **Input:** words = \[“k”,”ki”,”kir”,”kira”, “kiran”\]

// **Output:** “kiran”

// **Explanation:** “kiran” has prefixes “kira”, “kir”, “ki”, and “k”, and all of them appear in words.

// **Example 2:**

// **Input:** words = \[“a”, “banana”, “app”, “appl”, “ap”, “apply”, “apple”\]

// **Output:** “apple”

// **Explanation:** Both “apple” and “apply” have all their prefixes in words. However, “apple” is lexicographically smaller, so we return that.

// **Example 3:**

// **Input:** words = \[“abc”, “bc”, “ab”, “qwe”\]

// **Output:** “”

// **Constraints:**

// *   `1 <= words.length <= 10^5`
// *   `1 <= words[i].length <= 10^5`
// *   `1 <= sum(words[i].length) <= 10^5`




//   string longest_word(vector<string>& words) 

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
