// # [2950. Number of Divisible Substrings](https://leetcode.com/problems/number-of-divisible-substrings)

// ## Description

// Each character of the English alphabet has been mapped to a digit as shown below.

// <img alt="" src="https://fastly.jsdelivr.net/gh/doocs/leetcode@main/solution/2900-2999/2950.Number%20of%20Divisible%20Substrings/images/old_phone_digits.png" style="padding: 10px; width: 200px; height: 200px;" />

// A string is divisible if the sum of the mapped values of its characters is divisible by its length.

// Given a string s, return the number of divisible substrings of s.

// A substring is a contiguous non-empty sequence of characters within a string.

//
// Example 1:

// <table border="1" cellspacing="3" style="border-collapse: separate; text-align: center;">

// 			<th style="padding: 5px; border: 1px solid black;">Substring
// 			<th style="padding: 5px; border: 1px solid black;">Mapped
// 			<th style="padding: 5px; border: 1px solid black;">Sum
// 			<th style="padding: 5px; border: 1px solid black;">Length
// 			<th style="padding: 5px; border: 1px solid black;">Divisible?

// 			<td style="padding: 5px; border: 1px solid black;">a
// 			<td style="padding: 5px; border: 1px solid black;">1
// 			<td style="padding: 5px; border: 1px solid black;">1
// 			<td style="padding: 5px; border: 1px solid black;">1
// 			<td style="padding: 5px; border: 1px solid black;">Yes

// 			<td style="padding: 5px; border: 1px solid black;">s
// 			<td style="padding: 5px; border: 1px solid black;">7
// 			<td style="padding: 5px; border: 1px solid black;">7
// 			<td style="padding: 5px; border: 1px solid black;">1
// 			<td style="padding: 5px; border: 1px solid black;">Yes

// 			<td style="padding: 5px; border: 1px solid black;">d
// 			<td style="padding: 5px; border: 1px solid black;">2
// 			<td style="padding: 5px; border: 1px solid black;">2
// 			<td style="padding: 5px; border: 1px solid black;">1
// 			<td style="padding: 5px; border: 1px solid black;">Yes

// 			<td style="padding: 5px; border: 1px solid black;">f
// 			<td style="padding: 5px; border: 1px solid black;">3
// 			<td style="padding: 5px; border: 1px solid black;">3
// 			<td style="padding: 5px; border: 1px solid black;">1
// 			<td style="padding: 5px; border: 1px solid black;">Yes

// 			<td style="padding: 5px; border: 1px solid black;">as
// 			<td style="padding: 5px; border: 1px solid black;">1, 7
// 			<td style="padding: 5px; border: 1px solid black;">8
// 			<td style="padding: 5px; border: 1px solid black;">2
// 			<td style="padding: 5px; border: 1px solid black;">Yes

// 			<td style="padding: 5px; border: 1px solid black;">sd
// 			<td style="padding: 5px; border: 1px solid black;">7, 2
// 			<td style="padding: 5px; border: 1px solid black;">9
// 			<td style="padding: 5px; border: 1px solid black;">2
// 			<td style="padding: 5px; border: 1px solid black;">No

// 			<td style="padding: 5px; border: 1px solid black;">df
// 			<td style="padding: 5px; border: 1px solid black;">2, 3
// 			<td style="padding: 5px; border: 1px solid black;">5
// 			<td style="padding: 5px; border: 1px solid black;">2
// 			<td style="padding: 5px; border: 1px solid black;">No

// 			<td style="padding: 5px; border: 1px solid black;">asd
// 			<td style="padding: 5px; border: 1px solid black;">1, 7, 2
// 			<td style="padding: 5px; border: 1px solid black;">10
// 			<td style="padding: 5px; border: 1px solid black;">3
// 			<td style="padding: 5px; border: 1px solid black;">No

// 			<td style="padding: 5px; border: 1px solid black;">sdf
// 			<td style="padding: 5px; border: 1px solid black;">7, 2, 3
// 			<td style="padding: 5px; border: 1px solid black;">12
// 			<td style="padding: 5px; border: 1px solid black;">3
// 			<td style="padding: 5px; border: 1px solid black;">Yes

// 			<td style="padding: 5px; border: 1px solid black;">asdf
// 			<td style="padding: 5px; border: 1px solid black;">1, 7, 2, 3
// 			<td style="padding: 5px; border: 1px solid black;">13
// 			<td style="padding: 5px; border: 1px solid black;">4
// 			<td style="padding: 5px; border: 1px solid black;">No

// Input: word = "asdf"
// Output: 6
// Explanation: The table above contains the details about every substring of word, and we can see that 6 of them are divisible.

// Example 2:

// Input: word = "bdh"
// Output: 4
// Explanation: The 4 divisible substrings are: "b", "d", "h", "bdh".
// It can be shown that there are no other substrings of word that are divisible.

// Example 3:

// Input: word = "abcd"
// Output: 6
// Explanation: The 6 divisible substrings are: "a", "b", "c", "d", "ab", "cd".
// It can be shown that there are no other substrings of word that are divisible.

//
// Constraints:

// 	1  <= word.length  <= 2000
// 	word consists only of lowercase English letters.

// ```rust
// use std::collections::HashMap;

// impl Solution {
//     pub fn count_divisible_substrings(word: String) -> i32 {
//
//     }
// }
// ```

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_divisible_substrings(word: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_divisible_substrings_1() {
        assert_eq!(6, Solution::count_divisible_substrings("asdf".to_string()));
    }
    #[test]
    pub fn test_count_divisible_substrings_2() {
        assert_eq!(4, Solution::count_divisible_substrings("bdh".to_string()));
    }
    #[test]
    pub fn test_count_divisible_substrings_3() {
        assert_eq!(6, Solution::count_divisible_substrings("abcd".to_string()));
    }
}
