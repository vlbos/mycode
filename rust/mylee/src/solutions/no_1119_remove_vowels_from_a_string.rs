/*
Given a string S, remove the vowels 'a', 'e', 'i', 'o', and 'u' from it, and return the new string.

 

Example 1:
Input: "leetcodeisacommunityforcoders"
Output: "ltcdscmmntyfrcdrs"

Example 2:
Input: "aeiou"
Output: ""

 

Note:
	S consists of lowercase English letters only.
	1 <= S.length <= 1000


*/
pub struct Solution {}
impl Solution {
    pub fn remove_vowels(s: String) -> String {
        String::new()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test__1() {
        assert_eq!(String::new(), Solution::remove_vowels(String::new()));
    }
}
