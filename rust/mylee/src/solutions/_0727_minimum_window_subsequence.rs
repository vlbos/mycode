// # [727. Minimum Window Subsequence 🔒](https://leetcode.com/problems/minimum-window-subsequence)

// ## Description

//

// Given strings s1 and s2, return the minimum contiguous substring part of s1,
// so that s2 is a subsequence of the part.

// If there is no such window in s1 that covers all characters in s2,
// return the empty string  " ".
//  If there are multiple such minimum-length windows,
// return the one with the left-most starting index.

//
// Example 1:

//
// Input: s1 =  "abcdebdde", s2 =  "bde"
// Output:  "bcde"
// Explanation:
//  "bcde " is the answer because it occurs before  "bdde " which has the same length.
//  "deb " is not a smaller window because the elements of s2 in the window must occur in order.
//

// Example 2:

//
// Input: s1 =  "jmeqksfrsdcmsiwvaovztaqenprpvnbstl", s2 =  "u"
// Output:  ""
//

//
// Constraints:

//
// 	1 <= s1.length <= 2 * 104
// 	1 <= s2.length <= 100
// 	s1 and s2 consist of lowercase English letters.
//

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_window(s1: String, s2: String) -> String {
        let (m, n) = (s1.len(), s2.len());
        let mut f = vec![vec![0; n + 1]; m + 1];
        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                f[i + 1][j + 1] = if c1 == c2 {
                    if j == 0 {
                        i + 1
                    } else {
                        f[i][j]
                    }
                } else {
                    f[i][j + 1]
                };
            }
        }
        let bs2 = s2.as_bytes();
        let (mut p, mut k) = (0, m + 1);
        for (i, b1) in s1.bytes().enumerate() {
            if b1 == bs2[n - 1] && f[i + 1][n] > 0 {
                let j = f[i + 1][n] - 1;
                if i + 1 - j < k {
                    k = i + 1 - j;
                    p = j;
                }
            }
        }
        if k > m {
            String::new()
        } else {
            s1[p..p + k].to_owned()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_window_1() {
        assert_eq!(
            String::from("bcde"),
            Solution::min_window(String::from("abcdebdde"), String::from("bde"))
        );
    }
    #[test]
    pub fn test_min_window_2() {
        assert_eq!(
            String::from(""),
            Solution::min_window(
                String::from("jmeqksfrsdcmsiwvaovztaqenprpvnbstl"),
                String::from("u")
            )
        );
    }
}
