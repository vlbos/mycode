// ## [3481\. Apply Substitutions 🔒](https://leetcode.com/problems/apply-substitutions)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given a `replacements` mapping and a `text` string that may contain **placeholders** formatted as `%var%`, where each `var` corresponds to a key in the `replacements` mapping. Each replacement value may itself contain **one or more** such **placeholders**. Each **placeholder** is replaced by the value associated with its corresponding replacement key.

// Return the fully substituted `text` string which **does not** contain any **placeholders**.

// **Example 1:**

// **Input:** replacements = \[\["A","abc"\],\["B","def"\]\], text = "%A%\_%B%"

// **Output:** "abc\_def"

// **Explanation:**

// +   The mapping associates `"A"` with `"abc"` and `"B"` with `"def"`.
// +   Replace `%A%` with `"abc"` and `%B%` with `"def"` in the text.
// +   The final text becomes `"abc_def"`.

// **Example 2:**

// **Input:** replacements = \[\["A","bce"\],\["B","ace"\],\["C","abc%B%"\]\], text = "%A%\_%B%\_%C%"

// **Output:** "bce\_ace\_abcace"

// **Explanation:**

// +   The mapping associates `"A"` with `"bce"`, `"B"` with `"ace"`, and `"C"` with `"abc%B%"`.
// +   Replace `%A%` with `"bce"` and `%B%` with `"ace"` in the text.
// +   Then, for `%C%`, substitute `%B%` in `"abc%B%"` with `"ace"` to obtain `"abcace"`.
// +   The final text becomes `"bce_ace_abcace"`.

// **Constraints:**

// +   `1 <= replacements.length <= 10`
// +   Each element of `replacements` is a two-element list `[key, value]`, where:
//     +   `key` is a single uppercase English letter.
//     +   `value` is a non-empty string of at most 8 characters that may contain zero or more placeholders formatted as `%<key>%`.
// +   All replacement keys are unique.
// +   The `text` string is formed by concatenating all key placeholders (formatted as `%<key>%`) randomly from the replacements mapping, separated by underscores.
// +   `text.length == 4 * replacements.length - 1`
// +   Every placeholder in the `text` or in any replacement value corresponds to a key in the `replacements` mapping.
// +   There are no cyclic dependencies between replacement keys.

// string apply_substitutions(vector<vector<string>>& replacements, string text) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn apply_substitutions(replacements: Vec<Vec<String>>, text: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix_s;
    #[test]
    pub fn test_apply_substitutions_1() {
        assert_eq!(
            "abc_def".to_owned(),
            Solution::apply_substitutions(
                lc_matrix_s![["A", "abc"], ["B", "def"]],
                "%A%_%B%".to_owned()
            )
        );
    }
    #[test]
    pub fn test_apply_substitutions_2() {
        assert_eq!(
            "bce_ace_abcace".to_owned(),
            Solution::apply_substitutions(
                lc_matrix_s![["A", "bce"], ["B", "ace"], ["C", "abc%B%"]],
                "%A%_%B%_%C%".to_owned()
            )
        );
    }
}
