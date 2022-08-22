// 1788\. Maximize the Beauty of the Garden
// ========================================

// There is a garden of `n` flowers, and each flower has an integer beauty value. The flowers are arranged in a line. You are given an integer array `flowers` of size `n` and each `flowers[i]` represents the beauty of the `ith` flower.

// A garden is **valid** if it meets these conditions:

// *   The garden has at least two flowers.
// *   The first and the last flower of the garden have the same beauty value.

// As the appointed gardener, you have the ability to **remove** any (possibly none) flowers from the garden. You want to remove flowers in a way that makes the remaining garden **valid**. The beauty of the garden is the sum of the beauty of all the remaining flowers.

// Return the maximum possible beauty of some **valid** garden after you have removed any (possibly none) flowers.

// **Example 1:**

// **Input:** flowers = \[1,2,3,1,2\]
// **Output:** 8
// **Explanation:** You can produce the valid garden \[2,3,1,2\] to have a total beauty of 2 + 3 + 1 + 2 = 8.

// **Example 2:**

// **Input:** flowers = \[100,1,1,-3,1\]
// **Output:** 3
// **Explanation:** You can produce the valid garden \[1,1,1\] to have a total beauty of 1 + 1 + 1 = 3.

// **Example 3:**

// **Input:** flowers = \[-1,-2,0,-1\]
// **Output:** -2
// **Explanation:** You can produce the valid garden \[-1,-1\] to have a total beauty of -1 + -1 = -2.

// **Constraints:**

// *   `2 <= flowers.length <= 105`
// *   `-104 <= flowers[i] <= 104`
// *   It is possible to create a valid garden by removing some (possibly none) flowers.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

// int maximumBeauty(int[] flowers) {

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
