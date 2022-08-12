// 1087\. Brace Expansion
// ======================

// A string `S` represents a list of words.

// Each letter in the word has 1 or more options.  If there is one option, the letter is represented as is.
//  If there is more than one option, then curly braces delimit the options.  For example, `"{a,b,c}"` represents options `["a", "b", "c"]`.

// For example, `"{a,b,c}d{e,f}"` represents the list `["ade", "adf", "bde", "bdf", "cde", "cdf"]`.

// Return all words that can be formed in this manner, in lexicographical order.

// **Example 1:**

// **Input:** "{a,b}c{d,e}f"
// **Output:** \["acdf","acef","bcdf","bcef"\]

// **Example 2:**

// **Input:** "abcd"
// **Output:** \["abcd"\]

// **Note:**

// 1.  `1 <= S.length <= 50`
// 2.  There are no nested curly brackets.
// 3.  All characters inside a pair of consecutive opening and ending curly brackets are different.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn   expand(s: String) -> Vec<String> {
        let mut ans = vec![String::new()];
        let mut pre = 0;
        for (i, _) in s.match_indices(&['{', '}']) {
            if i > pre {
                let mut new_ans = Vec::new();
                for c in s[pre..i].split(",") {
                    for a in &ans {
                        new_ans.push(format!("{}{}", a, c));
                    }
                }
                ans = new_ans;
            }
            pre = i + 1;
        }
        if pre < s.len() {
            let mut new_ans = Vec::new();
            for c in s[pre..].split(",") {
                for a in &ans {
                    new_ans.push(format!("{}{}", a, c));
                }
            }
            ans = new_ans;
        }
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_expand_1() {
        assert_eq!(
            ["acdf", "acef", "bcdf", "bcef"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
            Solution::expand(String::from("{a,b}c{d,e}f"))
        );
    }
    #[test]
   pub fn  test_expand_2() {
        assert_eq!(
            vec!["abcd".to_string()],
            Solution::expand(String::from("abcd"))
        );
    }
}
