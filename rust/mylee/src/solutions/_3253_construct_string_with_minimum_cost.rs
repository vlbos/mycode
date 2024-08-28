// [3253\. Construct String with Minimum Cost (Easy)](https://leetcode.com/problems/construct-string-with-minimum-cost-easy/)

// Medium

// Hint
// You are given a string `target`, an array of strings `words`, and an integer array `costs`, both arrays of the same length.

// Imagine an empty string `s`.

// You can perform the following operation any number of times (including **zero**):

// *   Choose an index `i` in the range `[0, words.length - 1]`.
// *   Append `words[i]` to `s`.
// *   The cost of operation is `costs[i]`.

// Return the **minimum** cost to make `s` equal to `target`. If it's not possible, return -1.

// **Example 1:**

// **Input:** target = "abcdef", words = \["abdef","abc","d","def","ef"\], costs = \[100,1,1,10,5\]

// **Output:** 7

// **Explanation:**

// The minimum cost can be achieved by performing the following operations:

// *   Select index 1 and append `"abc"` to `s` at a cost of 1, resulting in `s = "abc"`.
// *   Select index 2 and append `"d"` to `s` at a cost of 1, resulting in `s = "abcd"`.
// *   Select index 4 and append `"ef"` to `s` at a cost of 5, resulting in `s = "abcdef"`.

// **Example 2:**

// **Input:** target = "aaaa", words = \["z","zz","zzz"\], costs = \[1,10,100\]

// **Output:** \-1

// **Explanation:**

// It is impossible to make `s` equal to `target`, so we return -1.

// **Constraints:**

// *   `1 <= target.length <= 2000`
// *   `1 <= words.length == costs.length <= 50`
// *   `1 <= words[i].length <= target.length`
// *   `target` and `words[i]` consist only of lowercase English letters.
// *   `1 <= costs[i] <= 105`

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let n = target.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 0..n {
            if dp[i] == i32::MAX {
                continue;
            }
            for (w, &c) in words.iter().zip(&costs) {
                let len = w.len();
                if i + len <= n && &target[i..i + len] == w {
                    dp[i + len] = dp[i + len].min(dp[i] + c);
                }
            }
        }
        if dp[n] == i32::MAX {
            -1
        } else {
            dp[n]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    pub fn test_minimum_cost_1() {
        assert_eq!(
            7,
            Solution::minimum_cost(
                String::from("abcdef"),
                lc_vec_s!["abdef", "abc", "d", "def", "ef"],
                vec![100, 1, 1, 10, 5]
            )
        );
    }

    #[test]
    pub fn test_minimum_cost_2() {
        assert_eq!(
            -1,
            Solution::minimum_cost(
                String::from("aaaa"),
                lc_vec_s!["z", "zz", "zzz"],
                vec![1, 10, 100]
            )
        );
    }
}
