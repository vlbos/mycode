// 1180\. Count Substrings with Only One Distinct Letter
// =====================================================

// Given a string `S`, return the number of substrings that have only **one distinct** letter.

// **Example 1:**

// **Input:** S = "aaaba"
// **Output:** 8
// **Explanation:** The substrings with one distinct letter are "aaa", "aa", "a", "b".
// "aaa" occurs 1 time.
// "aa" occurs 2 times.
// "a" occurs 4 times.
// "b" occurs 1 time.
// So the answer is 1 + 2 + 4 + 1 = 8.

// **Example 2:**

// **Input:** S = "aaaaaaaaaa"
// **Output:** 55

// **Constraints:**

// *   `1 <= S.length <= 1000`
// *   `S[i]` consists of only lowercase English letters.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Virtu Financial](https://leetcode.ca/tags/#Virtu%20Financial)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn count_letters(s: String) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;
        let mut pre = ' ';
        for c in s.chars() {
            cnt += 1;
            if pre != c {
                cnt = 1;
                pre = c;
            }
            ans += cnt;
        }
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_letters_1() {
        assert_eq!(8, Solution::count_letters(String::from("aaaba")));
    }
    #[test]
    pub fn test_count_letters_2() {
        assert_eq!(55, Solution::count_letters(String::from("aaaaaaaaaa")));
    }
}
