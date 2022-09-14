// 1062\. Longest Repeating Substring
// ==================================

// Given a string `S`, find out the length of the longest repeating substring(s). Return `0` if no repeating substring exists.

// **Example 1:**

// **Input:** "abcd"
// **Output:** 0
// **Explanation:** There is no repeating substring.

// **Example 2:**

// **Input:** "abbaba"
// **Output:** 2
// **Explanation:** The longest repeating substrings are "ab" and "ba", each of which occurs twice.

// **Example 3:**

// **Input:** "aabcaabdaab"
// **Output:** 3
// **Explanation:** The longest repeating substring is "aab", which occurs `3` times.

// **Example 4:**

// **Input:** "aaaaa"
// **Output:** 4
// **Explanation:** The longest repeating substring is "aaaa", which occurs twice.

// **Note:**

// 1.  The string `S` consists of only lowercase English letters from `'a'` \- `'z'`.
// 2.  `1 <= S.length <= 1500`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}



// impl Solution {
//     pub fn longest_repeating_substring(s: String) -> i32 {
//         let mut pre = ' ';
//         let mut cnt = 0;
//         let mut ans = 0;
//         for c in s.chars() {
//             if pre == c {
//                 cnt += 1;
//             } else if cnt > 0 {
//                 ans = ans.max(cnt + 1);
//             }
//             pre = c;
//         }
//         ans
//     }
// }

impl Solution {
    pub fn longest_repeating_substring(s: String) -> i32 {

    let  search=|mid: usize|{
        let mut seen = std::collections::HashSet::new();
        for i in 0..(s.len() - mid + 1) {
            let tmp = &s[i..i+mid];

            if seen.contains(tmp) {
                return true
            }

            seen.insert(tmp);
        }

        false
    };
        let mut lo = 1;
        let mut hi = s.len() as i32;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if search(mid as usize) {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        lo - 1
    }

}
#[cfg(test)]
mod test {
    use super::*;
// "aabcaabdaab"
// 3
    #[test]
    pub fn test_longest_repeating_substring_1() {
        assert_eq!(
            0,
            Solution::longest_repeating_substring(String::from("abcd"))
        );
    }
    #[test]
    pub fn test_longest_repeating_substring_2() {
        assert_eq!(
            2,
            Solution::longest_repeating_substring(String::from("abbaba"))
        );
    }
    #[test]
    pub fn test_longest_repeating_substring_3() {
        assert_eq!(
            3,
            Solution::longest_repeating_substring(String::from("aabcaabdaab"))
        );
    }
    #[test]
    pub fn test_longest_repeating_substring_4() {
        assert_eq!(
            4,
            Solution::longest_repeating_substring(String::from("aaaaa"))
        );
    }
}
