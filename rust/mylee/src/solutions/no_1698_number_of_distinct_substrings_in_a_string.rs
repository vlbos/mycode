// 1698\. Number of Distinct Substrings in a String
// ================================================

// Given a string `s`, return _the number of **distinct** substrings of_ `s`.

// A **substring** of a string is obtained by deleting any number of characters (possibly zero) from the front of the string and any number (possibly zero) from the back of the string.

// **Example 1:**

// **Input:** s = "aabbaba"
// **Output:** 21
// **Explanation:** The set of distinct strings is \["a","b","aa","bb","ab","ba","aab","abb","bba","aba","aabb","abba","bbab","baba","aabba","abbab","bbaba","aabbab","abbaba","aabbaba"\]

// **Example 2:**

// **Input:** s = "abcdefg"
// **Output:** 28

// **Constraints:**

// *   `1 <= s.length <= 500`
// *   `s` consists of lowercase English letters.

// **Follow up:** Can you solve this problem in `O(n)` time complexity?

// **Hints:**

// Hint 1

// Calculate the prefix hashing array for s.

// Hint 2

// Use the prefix hashing array to calculate the hashing value of each substring.

// Hint 3

// Compare the hashing values to determine the unique substrings.

// Hint 4

// There could be collisions if you use hashing, what about double hashing.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Dunzo](https://leetcode.ca/tags/#Dunzo) [Intuit](https://leetcode.ca/tags/#Intuit)

//  int countDistinct(String s)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}
