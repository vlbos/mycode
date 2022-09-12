// # [358. Rearrange String k Distance Apart](https://leetcode.com/problems/rearrange-string-k-distance-apart)

// [中文文档](/solution/0300-0399/0358.Rearrange%20String%20k%20Distance%20Apart/README.md)

// ## Description

// <p>Given a string <code>s</code> and an integer <code>k</code>, rearrange <code>s</code> such that the same characters are <strong>at least</strong> distance <code>k</code> from each other. If it is not possible to rearrange the string, return an empty string <code>&quot;&quot;</code>.</p>

// <p>&nbsp;</p>
// <p><strong>Example 1:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aabbcc&quot;, k = 3
// <strong>Output:</strong> &quot;abcabc&quot;
// <strong>Explanation:</strong> The same letters are at least a distance of 3 from each other.
// </pre>

// <p><strong>Example 2:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aaabc&quot;, k = 3
// <strong>Output:</strong> &quot;&quot;
// <strong>Explanation:</strong> It is not possible to rearrange the string.
// </pre>

// <p><strong>Example 3:</strong></p>

// <pre>
// <strong>Input:</strong> s = &quot;aaadbbcc&quot;, k = 2
// <strong>Output:</strong> &quot;abacabcd&quot;
// <strong>Explanation:</strong> The same letters are at least a distance of 2 from each other.
// </pre>

// <p>&nbsp;</p>
// <p><strong>Constraints:</strong></p>

// <ul>
// 	<li><code>1 &lt;= s.length &lt;= 3 * 10<sup>5</sup></code></li>
// 	<li><code>s</code> consists of only lowercase English letters.</li>
// 	<li><code>0 &lt;= k &lt;= s.length</code></li>
// </ul>

impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
use std::cmp::Reverse;
use std::collections::{BinaryHeap,VecDeque};

if k == 0 {
            return s;
        }

        let mut cnt = vec![0;26];
        let mut queue = BinaryHeap::new();
        let mut remain = VecDeque::new();
        
        for &u in s.as_bytes() {
            cnt[u as usize - 'a' as usize] += 1;
        }

        for i in 0..cnt.len() {
            if cnt[i] > 0 {
                queue.push((cnt[i], (i as u8 + b'a')));
            }
        }

        let mut ans = String::new();

        while let Some((v, u)) = queue.pop() {
            ans.push(u as char);
            remain.push_back((v-1, u));

            if remain.len() == k as usize {
                if let Some(&(t, k)) = remain.front() {
                    if t > 0 {
                        queue.push((t, k));
                    }
                }
                remain.pop_front();
            }
        }

        if ans.len() < s.len() {
            "".to_string()
        } else {
            ans
        }
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