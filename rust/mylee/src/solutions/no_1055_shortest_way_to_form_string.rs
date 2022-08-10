// 1055\. Shortest Way to Form String
// ==================================

// From any string, we can form a _subsequence_ of that string by deleting some number of characters (possibly no deletions).

// Given two strings `source` and `target`, return the minimum number of subsequences of `source` such that their concatenation equals `target`.
// If the task is impossible, return `-1`.

// **Example 1:**

// **Input:** source = "abc", target = "abcbc"
// **Output:** 2
// **Explanation:** The target "abcbc" can be formed by "abc" and "bc", which are subsequences of source "abc".

// **Example 2:**

// **Input:** source = "abc", target = "acdbc"
// **Output:** \-1
// **Explanation:** The target string cannot be constructed from the subsequences of source string due to the character "d" in target string.

// **Example 3:**

// **Input:** source = "xyz", target = "xzyxz"
// **Output:** 3
// **Explanation:** The target string can be constructed as follows "xz" + "y" + "xz".

// **Constraints:**

// *   Both the `source` and `target` strings consist of only lowercase English letters from "a"-"z".
// *   The lengths of `source` and `target` string are between `1` and `1000`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
 
#[allow(dead_code)] 
 pub struct Solution {}
impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let mut ans = 1;
        let mut pos = None;
        for c in target.chars() {
            if source.find(c).is_none() {
                return -1;
            }
            pos = source[if let Some(i) = pos { i + 1 } else { 0 }..].find(c);
            if pos.is_none() {
                ans += 1;
                pos = source.find(c);
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_way_1() {
        assert_eq!(
            2,
            Solution::shortest_way(String::from("abc"), String::from("abcbc"))
        );
    }
    #[test]
    fn test_shortest_way_2() {
        assert_eq!(
            -1,
            Solution::shortest_way(String::from("abc"), String::from("acdbc"))
        );
    }
    #[test]
    fn test_shortest_way_3() {
        assert_eq!(
            3,
            Solution::shortest_way(String::from("xyz"), String::from("xzyxz"))
        );
    }
}
