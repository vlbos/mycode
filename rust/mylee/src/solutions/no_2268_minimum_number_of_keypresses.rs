// # [2268. Minimum Number of Keypresses](https://leetcode.com/problems/minimum-number-of-keypresses)

// [中文文档](/solution/2200-2299/2268.Minimum%20Number%20of%20Keypresses/README.md)

// ## Description

// <p>You have a keypad with <code>9</code> buttons, numbered from <code>1</code> to <code>9</code>, each mapped to lowercase English letters. You can choose which characters each button is matched to as long as:</p>

// <ul>
// 	<li>All 26 lowercase English letters are mapped to.</li>
// 	<li>Each character is mapped to by <strong>exactly</strong> <code>1</code> button.</li>
// 	<li>Each button maps to <strong>at most</strong> <code>3</code> characters.</li>
// </ul>

// <p>To type the first character matched to a button, you press the button once. To type the second character, you press the button twice, and so on.</p>

// <p>Given a string <code>s</code>, return <em>the <strong>minimum</strong> number of keypresses needed to type </em><code>s</code><em> using your keypad.</em></p>

// <p><strong>Note</strong> that the characters mapped to by each button, and the order they are mapped in cannot be changed.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2268.Minimum%20Number%20of%20Keypresses/images/image-20220505184346-1.png" style="width: 300px; height: 293px;" />
// <pre>
// <strong>Input:</strong> s = &quot;apple&quot;
// <strong>Output:</strong> 5
// <strong>Explanation:</strong> One optimal way to setup your keypad is shown above.
// Type &#39;a&#39; by pressing button 1 once.
// Type &#39;p&#39; by pressing button 6 once.
// Type &#39;p&#39; by pressing button 6 once.
// Type &#39;l&#39; by pressing button 5 once.
// Type &#39;e&#39; by pressing button 3 once.
// A total of 5 button presses are needed, so return 5.
// </pre>

// <p><strong>Example 2:</strong></p>
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2268.Minimum%20Number%20of%20Keypresses/images/image-20220505203823-1.png" style="width: 300px; height: 288px;" />
// <pre>
// <strong>Input:</strong> s = &quot;abcdefghijkl&quot;
// <strong>Output:</strong> 15
// <strong>Explanation:</strong> One optimal way to setup your keypad is shown above.
// The letters &#39;a&#39; to &#39;i&#39; can each be typed by pressing a button once.
// Type &#39;j&#39; by pressing button 1 twice.
// Type &#39;k&#39; by pressing button 2 twice.
// Type &#39;l&#39; by pressing button 3 twice.
// A total of 15 button presses are needed, so return 15.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
// 	<li><code>s</code> consists of lowercase English letters.</li>
// </ul>

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def minimumKeypresses(self, s: str) -> int:

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
