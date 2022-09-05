// # [2268. Minimum Number of Keypresses](https://leetcode.com/problems/minimum-number-of-keypresses)

// ## Description

// You have a keypad with 9 buttons, numbered from 1 to 9, each mapped to lowercase English letters. You can choose which characters each button is matched to as long as:

//
// 	All 26 lowercase English letters are mapped to.
// 	Each character is mapped to by exactly 1 button.
// 	Each button maps to at most 3 characters.
//

// To type the first character matched to a button, you press the button once. To type the second character, you press the button twice, and so on.

// Given a string s, return the minimum number of keypresses needed to type s using your keypad.

// Note that the characters mapped to by each button, and the order they are mapped in cannot be changed.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2268.Minimum%20Number%20of%20Keypresses/images/image-20220505184346-1.png" style="width: 300px; height: 293px;" />
//
// Input: s = &quot;apple&quot;
// Output: 5
// Explanation: One optimal way to setup your keypad is shown above.
// Type 'a' by pressing button 1 once.
// Type 'p' by pressing button 6 once.
// Type 'p' by pressing button 6 once.
// Type 'l' by pressing button 5 once.
// Type 'e' by pressing button 3 once.
// A total of 5 button presses are needed, so return 5.
//

// Example 2:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2268.Minimum%20Number%20of%20Keypresses/images/image-20220505203823-1.png" style="width: 300px; height: 288px;" />
//
// Input: s = &quot;abcdefghijkl&quot;
// Output: 15
// Explanation: One optimal way to setup your keypad is shown above.
// The letters 'a' to 'i' can each be typed by pressing a button once.
// Type 'j' by pressing button 1 twice.
// Type 'k' by pressing button 2 twice.
// Type 'l' by pressing button 3 twice.
// A total of 15 button presses are needed, so return 15.
//

// Constraints:

//
// 	1 <= s.length <= 105
// 	s consists of lowercase English letters.
//

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
