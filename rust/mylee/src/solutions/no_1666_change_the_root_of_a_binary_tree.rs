// 1666\. Change the Root of a Binary Tree
// =======================================

// Given the `root` of a binary tree and a `leaf` node, reroot the tree so that the `leaf` is the new root.

// You can reroot the tree with the following steps for each node `cur` on the path **starting from the** `leaf` up to the `root`​​​ **excluding the root**:

// 1.  If `cur` has a left child, then that child becomes `cur`'s right child. Note that it is guaranteed that `cur` will have at most one child.
// 2.  `cur`'s original parent becomes `cur`'s left child.

// Return _the new root_ _of the rerooted tree._

// **Note:** Ensure that your solution sets the `Node.parent` pointers correctly after rerooting or you will receive "Wrong Answer".

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/11/24/fliptree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], leaf = 7
// **Output:** \[7,2,null,5,4,3,6,null,null,null,1,null,null,0,8\]

// **Example 2:**

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], leaf = 0
// **Output:** \[0,1,null,3,8,5,null,null,null,6,2,null,null,7,4\]

// **Constraints:**

// *   The number of nodes in the tree is in the range `[2, 100]`.
// *   `-109 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   `leaf` exist in the tree.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//  Node flipBinaryTree(Node root, Node leaf)

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
