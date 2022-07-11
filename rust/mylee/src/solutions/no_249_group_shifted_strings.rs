// 249\. Group Shifted Strings
// ===========================

// Given a string, we can "shift" each of its letter to its successive letter, for example: `"abc" -> "bcd"`. We can keep "shifting" which forms the sequence:

// "abc" -> "bcd" -> ... -> "xyz"

// Given a list of strings which contains only lowercase alphabets, group all strings that belong to the same shifting sequence.

// **Example:**

// **Input:** `["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"],`
// **Output:**
// \[
//   \["abc","bcd","xyz"\],
//   \["az","ba"\],
//   \["acef"\],
//   \["a","z"\]
// \]

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)
struct Solution;
// @lc code=start
use std::collections::HashMap;

const A_CHARCODE: i32 = 'a' as i32;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut dict = HashMap::<String, Vec<String>>::new();
        for s in strings {
            let norm = Solution::shift_normalize(&s);
            dict.entry(norm)
                .and_modify(|v| v.push(s.clone()))
                .or_insert_with(|| vec![s]);
        }
        dict.values().cloned().collect::<Vec<Vec<String>>>()
    }

    fn shift(ch: char, times: i32) -> char {
        (A_CHARCODE + ((ch as i32) - A_CHARCODE + times).rem_euclid(26)) as u8 as char
    }

    fn shift_normalize(string: &String) -> String {
        if string.is_empty() {
            return string.to_string();
        }
        let mut times = 0;
        let chars = string.chars().collect::<Vec<_>>();
        let mut temp_char = chars[0];
        while temp_char != 'a' {
            temp_char = Solution::shift(temp_char, -1);
            times -= 1;
        }
        chars
            .iter()
            .map(|ch| Solution::shift(*ch, times))
            .collect::<String>()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::util::test_tools::{assert_nested_equivalent, map_nested_to_string, map_to_string};

    #[test]
    fn test_group_strings() {
        let src = map_to_string(&["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"]);
        let tar = map_nested_to_string(&[
            vec!["abc", "bcd", "xyz"],
            vec!["az", "ba"],
            vec!["acef"],
            vec!["a", "z"],
        ]);
        assert_nested_equivalent(&Solution::group_strings(src), &tar);
    }
}
