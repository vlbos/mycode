// # [2330. Valid Palindrome IV](https://leetcode.com/problems/valid-palindrome-iv)

// [中文文档](/solution/2300-2399/2330.Valid%20Palindrome%20IV/README.md)

// ## Description

// <p>You are given a <strong>0-indexed</strong> string <code>s</code> consisting of only lowercase English letters. In one operation, you can change <strong>any</strong> character of <code>s</code> to any <strong>other</strong> character.</p>

// <p>Return <code>true</code><em> if you can make </em><code>s</code><em> a palindrome after performing <strong>exactly</strong> one or two operations, or return </em><code>false</code><em> otherwise.</em></p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;abcdba&quot;
// <strong>Output:</strong> true
// <strong>Explanation:</strong> One way to make s a palindrome using 1 operation is:
// - Change s[2] to &#39;d&#39;. Now, s = &quot;abddba&quot;.
// One operation could be performed to make s a palindrome so return true.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aa&quot;
// <strong>Output:</strong> true
// <strong>Explanation:</strong> One way to make s a palindrome using 2 operations is:
// - Change s[0] to &#39;b&#39;. Now, s = &quot;ba&quot;.
// - Change s[1] to &#39;b&#39;. Now, s = &quot;bb&quot;.
// Two operations could be performed to make s a palindrome so return true.
// </pre>

// <p><strong>Example 3:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;abcdef&quot;
// <strong>Output:</strong> false
// <strong>Explanation:</strong> It is not possible to make s a palindrome using one or two operations so return false.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
// 	<li><code>s</code> consists only of lowercase English letters.</li>
// </ul>

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def makePalindrome(self, s: str) -> bool:

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
