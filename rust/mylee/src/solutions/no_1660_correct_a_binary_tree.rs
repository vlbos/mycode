// 1660\. Correct a Binary Tree
// ============================

// You have a binary tree with a small defect.
// There is **exactly one** invalid node where its right child incorrectly points to another node at the **same depth**
// but to the **invalid node's right**.

// Given the root of the binary tree with this defect, `root`,
// return _the root of the binary tree after **removing** this invalid node **and every node underneath it** (minus the node it incorrectly points to)._

// **Custom testing:**

// The test input is read as 3 lines:

// *   `TreeNode root`
// *   `int fromNode` (**not available to** `correctBinaryTree`)
// *   `int toNode` (**not available to** `correctBinaryTree`)

// After the binary tree rooted at `root` is parsed,
// the `TreeNode` with value of `fromNode` will have its right child pointer pointing to the `TreeNode` with a value of `toNode`.
// Then, `root` is passed to `correctBinaryTree`.

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

//  TreeNode correct_binary_tree(TreeNode root)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn correct_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q = std::collections::VecDeque::from([root.clone()]);
        while !q.is_empty() {
            let n = q.len();
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                if node.as_ref().unwrap().borrow().right.is_some() {
                    if node
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .is_some()
                        && q.contains(
                            &node
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .right
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .right,
                        )
                    {
                        node.as_ref().unwrap().borrow_mut().right = None;
                        return root;
                    }
                    q.push_back(node.as_ref().unwrap().borrow().right.clone());
                }
                if node.as_ref().unwrap().borrow().left.is_some() {
                    if node
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .is_some()
                        && q.contains(
                            &node
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .left
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .right,
                        )
                    {
                        node.as_ref().unwrap().borrow_mut().left = None;
                        return root;
                    }
                    q.push_back(node.as_ref().unwrap().borrow().left.clone());
                }
            }
        }
        root
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node = root.as_ref().unwrap().borrow();
            if node.val == v {
                return root.clone();
            }
            let left = dfs(&node.left, v);
            if left.is_some() {
                return left;
            }
            dfs(&node.right, v)
        } else {
            None
        }
    }
    #[test]
    pub fn test_correct_binary_tree_1() {
        let mut t = tree![1, 2, 3];
        let mut n2 = dfs(&t, 2);
        let n3 = dfs(&t, 3);
        n2.as_mut().unwrap().borrow_mut().right = n3;
        assert_eq!(tree![1, null, 3], Solution::correct_binary_tree(t));
    }
    #[test]
    pub fn test_correct_binary_tree_2() {
        let mut t = tree![8, 3, 1, 7, null, 9, 4, 2, null, null, null, 5, 6];
        let mut n7 = dfs(&t, 7);
        let n4 = dfs(&t, 4);
        n7.as_mut().unwrap().borrow_mut().right = n4;
        assert_eq!(
            tree![8, 3, 1, null, null, 9, 4, null, null, 5, 6],
            Solution::correct_binary_tree(t)
        );
    }
}
