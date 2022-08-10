// 266\. Palindrome Permutation
// ============================

// Given a string, determine if a permutation of the string could form a palindrome.

// **Example 1:**

// **Input:** `"code"`
// **Output:** false

// **Example 2:**

// **Input:** `"aab"`
// **Output:** true

// **Example 3:**

// **Input:** `"carerac"`
// **Output:** true

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Uber](https://leetcode.ca/tags/#Uber)
// @lc code=start
// use std::collections::HashMap;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        // let mut dict = HashMap::<char, usize>::new();
        // for c in s.chars() {
        //     dict.entry(c).and_modify(|c| *c += 1).or_insert(1);
        // }
        // let mut can_odd = (s.len() & 1) == 1;
        // for v in dict.values() {
        //     if ((*v) & 1) == 1 {
        //         if can_odd {
        //             can_odd = false;
        //         } else {
        //             return false;
        //         }
        //     }
        // }
        // true
        let mut cnt = std::collections::HashMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        cnt.values().filter(|x| *x % 2 == 1).count() < 2
    }
}
// @lc code=end
 
#[allow(dead_code)] 
 struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_permute_palindrome() {
        assert!(!Solution::can_permute_palindrome(String::from("code")));
        assert!(Solution::can_permute_palindrome(String::from("aab")));
        assert!(Solution::can_permute_palindrome(String::from("carerac")));
    }
}
