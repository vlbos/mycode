// # [471. Encode String with Shortest Length](https://leetcode.com/problems/encode-string-with-shortest-length)

// ## Description

// Given a string s, encode the string such that its encoded length is the shortest.

// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. k should be a positive integer.

// If an encoding process does not make the string shorter, then do not encode it. If there are several solutions, return any of them.

// Example 1:

//
// Input: s = "aaa"
// Output: "aaa"
// Explanation: There is no way to encode it such that it is shorter than the input string, so we do not encode it.
//

// Example 2:

//
// Input: s = "aaaaa"
// Output: "5[a]"
// Explanation: "5[a]" is shorter than "aaaaa" by 1 character.
//

// Example 3:

//
// Input: s = "aaaaaaaaaa"
// Output: "10[a]"
// Explanation: "a9[a]" or "9[a]a" are also valid solutions, both of them have the same length = 5, which is the same as "10[a]".
//

// Constraints:

//
// 	1 <= s.length <= 150
// 	s consists of only lowercase English letters.
//

impl Solution {
    pub fn encode(s: String) -> String {
        use std::collections::HashMap;
        fn dp(s: String, cache: &mut HashMap<String, String>) -> String {
            if let Some(ans) = cache.get(&s) {
                return ans.to_string();
            }

            let n = s.len();
            let mut ans = s.to_string();

            for i in 1..n {
                if n % i == 0 {
                    let j = n / i;

                    if (0..j).all(|k| s[0..i] == s[k * i..(k + 1) * i]) {
                        let t = format!("{}[{}]", j, dp(s[0..i].to_string(), cache));

                        if t.len() < ans.len() {
                            ans = t;
                        }
                    }
                }
            }

            for i in 1..n {
                let t = format!(
                    "{}{}",
                    dp(s[0..i].to_string(), cache),
                    dp(s[i..].to_string(), cache)
                );

                if t.len() < ans.len() {
                    ans = t;
                }
            }

            cache.insert(s, ans.to_string());

            ans
        }
        dp(s, &mut HashMap::new())
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::tree;
    #[test]
    pub fn test_read() {
        // assert_eq!(
        //     Solution::read(tree![1, 2, 3, 4, 5]),
        //     tree![4, 5, 2, null, null, 3, 1]
        // );
    }
}
