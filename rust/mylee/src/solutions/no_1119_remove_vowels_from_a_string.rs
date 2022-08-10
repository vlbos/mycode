// 1119\. Remove Vowels from a String
// ==================================

// Given a string `S`, remove the vowels `'a'`, `'e'`, `'i'`, `'o'`, and `'u'` from it, and return the new string.

// **Example 1:**

// **Input:** "leetcodeisacommunityforcoders"
// **Output:** "ltcdscmmntyfrcdrs"

// **Example 2:**

// **Input:** "aeiou"
// **Output:** ""

// **Note:**

// 1.  `S` consists of lowercase English letters only.
// 2.  `1 <= S.length <= 1000`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)
#[allow(dead_code)]
pub  struct Solution {}
impl Solution {
    pub fn remove_vowels(mut s: String) -> String {
        for c in "aeiou".chars() {
            s = s.replace(c.to_string().as_str(), "");
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_vowels_1() {
        assert_eq!(
            String::from("ltcdscmmntyfrcdrs"),
            Solution::remove_vowels(String::from("leetcodeisacommunityforcoders"))
        );
    }
    #[test]
    fn test_remove_vowels_2() {
        assert_eq!(
            String::new(),
            Solution::remove_vowels(String::from("aeiou"))
        );
    }
}
