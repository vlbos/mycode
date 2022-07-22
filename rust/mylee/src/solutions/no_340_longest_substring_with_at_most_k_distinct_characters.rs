// 340\. Longest Substring with At Most K Distinct Characters
// ==========================================================

// Given a string, find the length of the longest substring T that contains at most _k_ distinct characters.

// **Example 1:**

// **Input:** s = "eceba", k = 2
// **Output:** 3
// **Explanation:** T is "ece" which its length is 3.

// **Example 2:**

// **Input:** s = "aa", k = 1
// **Output:** 2
// **Explanation:** T is "aa" which its length is 2.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [AppDynamics](https://leetcode.ca/tags/#AppDynamics) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Citadel](https://leetcode.ca/tags/#Citadel) [Coupang](https://leetcode.ca/tags/#Coupang) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        // use std::collections::HashMap;
        // if k <= 0 {
        //     return k;
        // }
        // let chars = s.chars().collect::<Vec<char>>();
        // if chars.len() <= 1 {
        //     return chars.len() as i32;
        // }
        // let mut unique = HashMap::<char, usize>::new();
        // let mut max_len = 0usize;
        // let mut left = -1i32;
        // for right in 0..chars.len() {
        //     let ch = chars[right];
        //     unique.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        //     while unique.len() > k as usize {
        //         left += 1;
        //         let lch = chars[left as usize];
        //         {
        //             let count = unique[&lch];
        //             if count == 1 {
        //                 unique.remove(&lch);
        //             } else {
        //                 unique.insert(lch, count - 1);
        //             }
        //         }
        //     }
        //     max_len = usize::max(max_len, (right as i32 - left) as usize);
        // }
        // max_len as i32
        if k == 0 {
            return k;
        }
        if s.len() < 2 {
            return s.len() as i32;
        }
        let bs = s.as_bytes();
        let mut l = 0;
        let mut cnt = std::collections::HashMap::new();
        let mut ans = 0;
        for (r, b) in s.bytes().enumerate() {
            *cnt.entry(b).or_insert(0) += 1;
            while cnt.len() as i32 > k {
                *cnt.entry(bs[l]).or_insert(0) -= 1;
                if *cnt.get(&bs[l]).unwrap_or(&0) == 0 {
                    cnt.remove(&bs[l]);
                }
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }
        ans as _
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_k_distinct_1() {
        assert_eq!(
            Solution::length_of_longest_substring_k_distinct(String::from("eceba"), 2),
            3
        );
    }

    #[test]
    fn test_length_of_longest_substring_k_distinct_2() {
        assert_eq!(
            Solution::length_of_longest_substring_k_distinct(String::from("aa"), 1),
            2
        );
    }
}
