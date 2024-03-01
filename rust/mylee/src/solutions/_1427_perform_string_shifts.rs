// 1427\. Perform String Shifts
// ============================

// You are given a string `s` containing lowercase English letters, and a matrix `shift`, where `shift[i] = [direction, amount]`:

// *   `direction` can be `0` (for left shift) or `1` (for right shift).
// *   `amount` is the amount by which string `s` is to be shifted.
// *   A left shift by 1 means remove the first character of `s` and append it to the end.
// *   Similarly, a right shift by 1 means remove the last character of `s` and add it to the beginning.

// Return the final string after all operations.

// **Example 1:**

// **Input:** s = "abc", shift = \[\[0,1\],\[1,2\]\]
// **Output:** "cab"
// **Explanation:**
// \[0,1\] means shift to left by 1. "abc" -> "bca"
// \[1,2\] means shift to right by 2. "bca" -> "cab"

// **Example 2:**

// **Input:** s = "abcdefg", shift = \[\[1,1\],\[1,1\],\[0,2\],\[1,3\]\]
// **Output:** "efgabcd"
// **Explanation:**
// \[1,1\] means shift to right by 1. "abcdefg" -> "gabcdef"
// \[1,1\] means shift to right by 1. "gabcdef" -> "fgabcde"
// \[0,2\] means shift to left by 2. "fgabcde" -> "abcdefg"
// \[1,3\] means shift to right by 3. "abcdefg" -> "efgabcd"

// **Constraints:**

// *   `1 <= s.length <= 100`
// *   `s` only contains lower case English letters.
// *   `1 <= shift.length <= 100`
// *   `shift[i].length == 2`
// *   `0 <= shift[i][0] <= 1`
// *   `0 <= shift[i][1] <= 100`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Goldman Sachs](https://leetcode.ca/tags/#Goldman%20Sachs)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut ans = s;
        for sh in &shift {
            let mut i = sh[1] as usize;
            if sh[0] > 0 {
                i = n - i;
            }
            ans = format!("{}{}", &ans[i..], &ans[..i]);
        }
        ans
    }
}

// impl Solution {
//     pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
//  let mut count = 0;
//         for v in shift {
//             if v[0] == 1 {
//                 count += v[1];
//             }
//             else {
//                 count -= v[1];
//             }
//         }
//         let size = s.len() as i32;
//         count = (count % size + size) % size;
//         let mut ret = String::new();
//         ret.push_str(&s[((size-count) as usize)..]);
//         ret.push_str(&s[0..((size-count) as usize)]);
//         ret
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    // "a"
    // [[1,1],[1,1],[0,2],[1,3],[0,0]]
    #[test]
    pub fn test_string_shift_1() {
        assert_eq!(
            String::from("cab"),
            Solution::string_shift(String::from("abc"), vec![vec![0, 1], vec![1, 2]])
        );
    }
    #[test]
    pub fn test_string_shift_2() {
        assert_eq!(
            String::from("efgabcd"),
            Solution::string_shift(
                String::from("abcdefg"),
                vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]
            )
        );
    }
}
