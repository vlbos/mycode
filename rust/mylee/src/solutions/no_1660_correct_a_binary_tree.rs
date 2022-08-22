// 1660\. Correct a Binary Tree
// ============================

// You have a binary tree with a small defect. There is **exactly one** invalid node where its right child incorrectly points to another node at the **same depth** but to the **invalid node's right**.

// Given the root of the binary tree with this defect, `root`, return _the root of the binary tree after **removing** this invalid node **and every node underneath it** (minus the node it incorrectly points to)._

// **Custom testing:**

// The test input is read as 3 lines:

// *   `TreeNode root`
// *   `int fromNode` (**not available to** `correctBinaryTree`)
// *   `int toNode` (**not available to** `correctBinaryTree`)

// After the binary tree rooted at `root` is parsed, the `TreeNode` with value of `fromNode` will have its right child pointer pointing to the `TreeNode` with a value of `toNode`. Then, `root` is passed to `correctBinaryTree`.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2020/10/22/ex1v2.png)**

// **Input:** root = \[1,2,3\], fromNode = 2, toNode = 3
// **Output:** \[1,null,3\]
// **Explanation:** The node with value 2 is invalid, so remove it.

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2020/10/22/ex2v3.png)**

// **Input:** root = \[8,3,1,7,null,9,4,2,null,null,null,5,6\], fromNode = 7, toNode = 4
// **Output:** \[8,3,1,null,null,9,4,null,null,5,6\]
// **Explanation:** The node with value 7 is invalid, so remove it and the node underneath it, node 2.

// **Constraints:**

// *   The number of nodes in the tree is in the range `[3, 104]`.
// *   `-109 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   `fromNode != toNode`
// *   `fromNode` and `toNode` will exist in the tree and will be on the same depth.
// *   `toNode` is to the **right** of `fromNode`.
// *   `fromNode.right` is `null` in the initial tree from the test data.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//  TreeNode correctBinaryTree(TreeNode root)

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
