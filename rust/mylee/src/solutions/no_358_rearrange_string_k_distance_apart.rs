// # [358. Rearrange String k Distance Apart](https://leetcode.com/problems/rearrange-string-k-distance-apart)

// [中文文档](/solution/0300-0399/0358.Rearrange%20String%20k%20Distance%20Apart/README.md)

// ## Description

// Given a string s and an integer k, rearrange s such that the same characters are at least distance k from each other. If it is not possible to rearrange the string, return an empty string "".

// Example 1:

//
// Input: s = "aabbcc", k = 3
// Output: "abcabc"
// Explanation: The same letters are at least a distance of 3 from each other.
//

// Example 2:

//
// Input: s = "aaabc", k = 3
// Output: ""
// Explanation: It is not possible to rearrange the string.
//

// Example 3:

//
// Input: s = "aaadbbcc", k = 2
// Output: "abacabcd"
// Explanation: The same letters are at least a distance of 2 from each other.
//

// Constraints:

//
// 	1 <= s.length <= 3 * 105
// 	s consists of only lowercase English letters.
// 	0 <= k <= s.length
//

impl Solution {
    pub fn rearrange_string(s: String, k: i32) -> String {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, VecDeque};

        if k == 0 {
            return s;
        }

        let mut cnt = vec![0; 26];
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
            remain.push_back((v - 1, u));

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
