// 1772\. Sort Features by Popularity
// ==================================

// You are given a string array `features` where `features[i]` is a single word that represents the name of a feature of the latest product you are working on. You have made a survey where users have reported which features they like. You are given a string array `responses`, where each `responses[i]` is a string containing space-separated words.

// The **popularity** of a feature is the number of `responses[i]` that contain the feature. You want to sort the features in non-increasing order by their popularity. If two features have the same popularity, order them by their original index in `features`. Notice that one response could contain the same feature multiple times; this feature is only counted once in its popularity.

// Return _the features in sorted order._

// **Example 1:**

// **Input:** features = \["cooler","lock","touch"\], responses = \["i like cooler cooler","lock touch cool","locker like touch"\]
// **Output:** \["touch","cooler","lock"\]
// **Explanation:** appearances("cooler") = 1, appearances("lock") = 1, appearances("touch") = 2. Since "cooler" and "lock" both had 1 appearance, "cooler" comes first because "cooler" came first in the features array.

// **Example 2:**

// **Input:** features = \["a","aa","b","c"\], responses = \["a","a aa","a a a a a","b a"\]
// **Output:** \["a","aa","b","c"\]

// **Constraints:**

// *   `1 <= features.length <= 104`
// *   `1 <= features[i].length <= 10`
// *   `features` contains no duplicates.
// *   `features[i]` consists of lowercase letters.
// *   `1 <= responses.length <= 102`
// *   `1 <= responses[i].length <= 103`
// *   `responses[i]` consists of lowercase letters and spaces.
// *   `responses[i]` contains no two consecutive spaces.
// *   `responses[i]` has no leading or trailing spaces.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

// String[] sortFeatures(String[] features, String[] responses)

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
