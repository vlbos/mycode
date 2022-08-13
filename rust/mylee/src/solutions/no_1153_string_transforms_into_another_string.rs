// 1153\. String Transforms Into Another String
// ============================================

// Given two strings `str1` and `str2` of the same length, determine whether you can transform `str1` into `str2` by doing **zero or more** _conversions_.

// In one conversion you can convert **all** occurrences of one character in `str1` to **any** other lowercase English character.

// Return `true` if and only if you can transform `str1` into `str2`.

// **Example 1:**

// **Input:** str1 = "aabcc", str2 = "ccdee"
// **Output:** true
// **Explanation:** Convert 'c' to 'e' then 'b' to 'd' then 'a' to 'c'. Note that the order of conversions matter.

// **Example 2:**

// **Input:** str1 = "leetcode", str2 = "codeleet"
// **Output:** false
// **Explanation:** There is no way to transform str1 to str2.

// **Note:**

// 1.  `1 <= str1.length == str2.length <= 10^4`
// 2.  Both `str1` and `str2` contain only lowercase English letters.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn can_convert(str1: String, str2: String) -> bool {
        if str1 == str2 {
            return true;
        }
        let mut m = std::collections::HashMap::new();
        for (c1, c2) in str1.chars().zip(str2.chars()) {
            if let Some(&c) = m.get(&c1) {
                if c != c2 {
                    return false;
                }
            }
            m.insert(c1, c2);
        }
        m.values()
            .cloned()
            .collect::<std::collections::HashSet<char>>()
            .len()
            < 26
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_can_convert_1() {
        assert!(Solution::can_convert(
            String::from("aabcc"),
            String::from("ccdee")
        ));
    }
    #[test]
    pub fn test_can_convert_2() {
        assert!(!Solution::can_convert(
            String::from("leetcode"),
            String::from("codeleet")
        ));
    }
}
