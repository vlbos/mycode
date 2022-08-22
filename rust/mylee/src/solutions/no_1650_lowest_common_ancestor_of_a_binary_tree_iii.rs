// 1650\. Lowest Common Ancestor of a Binary Tree III
// ==================================================

// Given two nodes of a binary tree `p` and `q`, return _their lowest common ancestor (LCA)_.

// Each node will have a reference to its parent node. The definition for `Node` is below:

// class Node {
//     public int val;
//     public Node left;
//     public Node right;
//     public Node parent;
// }

// According to the **[definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor)**: "The lowest common ancestor of two nodes p and q in a tree T is the lowest node that has both p and q as descendants (where we allow **a node to be a descendant of itself**)."

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 1
// **Output:** 3
// **Explanation:** The LCA of nodes 5 and 1 is 3.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 4
// **Output:** 5
// **Explanation:** The LCA of nodes 5 and 4 is 5 since a node can be a descendant of itself according to the LCA definition.

// **Example 3:**

// **Input:** root = \[1,2\], p = 1, q = 2
// **Output:** 1

// **Constraints:**

// *   The number of nodes in the tree is in the range `[2, 105]`.
// *   `-109 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   `p != q`
// *   `p` and `q` exist in the tree.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Facebook](https://leetcode.ca/tags/#Facebook) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft)

// Node lowestCommonAncestor(Node p, Node q)

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
