// # [471. Encode String with Shortest Length](https://leetcode.com/problems/encode-string-with-shortest-length)

// ## Description

// <p>Given a string <code>s</code>, encode the string such that its encoded length is the shortest.</p>

// <p>The encoding rule is: <code>k[encoded_string]</code>, where the <code>encoded_string</code> inside the square brackets is being repeated exactly <code>k</code> times. <code>k</code> should be a positive integer.</p>

// <p>If an encoding process does not make the string shorter, then do not encode it. If there are several solutions, return <strong>any of them</strong>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aaa&quot;
// <strong>Output:</strong> &quot;aaa&quot;
// <strong>Explanation:</strong> There is no way to encode it such that it is shorter than the input string, so we do not encode it.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aaaaa&quot;
// <strong>Output:</strong> &quot;5[a]&quot;
// <strong>Explanation:</strong> &quot;5[a]&quot; is shorter than &quot;aaaaa&quot; by 1 character.
// </pre>

// <p><strong>Example 3:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aaaaaaaaaa&quot;
// <strong>Output:</strong> &quot;10[a]&quot;
// <strong>Explanation:</strong> &quot;a9[a]&quot; or &quot;9[a]a&quot; are also valid solutions, both of them have the same length = 5, which is the same as &quot;10[a]&quot;.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= s.length &lt;= 150</code></li>
// 	<li><code>s</code> consists of only lowercase English letters.</li>
// </ul>

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
    use super::*;
    // use crate::tree;
    #[test]
    pub fn test_read() {
        // assert_eq!(
        //     Solution::read(tree![1, 2, 3, 4, 5]),
        //     tree![4, 5, 2, null, null, 3, 1]
        // );
    }
}