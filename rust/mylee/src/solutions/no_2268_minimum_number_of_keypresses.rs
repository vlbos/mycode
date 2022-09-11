// # [2268. Minimum Number of Keypresses](https://leetcode.com/problems/minimum-number-of-keypresses)

// ## Description

// You have a keypad with 9 buttons, numbered from 1 to 9, each mapped to lowercase English letters.
// You can choose which characters each button is matched to as long as:

// 	All 26 lowercase English letters are mapped to.
// 	Each character is mapped to by exactly 1 button.
// 	Each button maps to at most 3 characters.

// To type the first character matched to a button, you press the button once.
// To type the second character, you press the button twice, and so on.

// Given a string s, return the minimum number of keypresses needed to type s using your keypad.

// Note that the characters mapped to by each button, and the order they are mapped in cannot be changed.

// Example 1:
// <img src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2200-2299/2268.Minimum%20Number%20of%20Keypresses/images/image-20220505184346-1.png" style="width: 300px; height: 293px;" />
//
// Input: s = "apple"
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
// Input: s = "abcdefghijkl"
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
// 	1 <= s.length <= 10^5
// 	s consists of lowercase English letters.
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def minimum_keypresses(self, s: str) -> int:

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn minimum_keypresses(s: String) -> i32 {
        if s.len() < 10 {
            return s.len() as i32;
        }
        let mut cnt = std::collections::HashMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut cnt: Vec<i32> = cnt.iter().map(|(_, &v)| v).collect();
        cnt.sort_by(|a, b| b.cmp(&a));
        let ans = cnt[..9].iter().sum::<i32>();
        if cnt.len() > 18 {
            ans + cnt[9..18].iter().sum::<i32>() * 2 + cnt[18..].iter().sum::<i32>() * 3
        } else {
            ans + cnt[9..].iter().sum::<i32>() * 2
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_minimum_keypresses_1() {
        assert_eq!(5, Solution::minimum_keypresses("apple".to_string()));
    }
    #[test]
    pub fn test_minimum_keypresses_2() {
        assert_eq!(15, Solution::minimum_keypresses("abcdefghijkl".to_string()));
    }
}
