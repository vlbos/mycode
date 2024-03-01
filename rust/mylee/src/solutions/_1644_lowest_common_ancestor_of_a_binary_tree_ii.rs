// 1644\. Lowest Common Ancestor of a Binary Tree II
// =================================================

// Given the `root` of a binary tree, return _the lowest common ancestor (LCA) of two given nodes,_ `p` _and_ `q`.
// If either node `p` or `q` **does not exist** in the tree, return `null`. All values of the nodes in the tree are **unique**.

// According to the **[definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor)**:
// "The lowest common ancestor of two nodes `p` and `q` in a binary tree `T` is the lowest node that has both `p` and `q` as **descendants**
// (where we allow **a node to be a descendant of itself**)".
// A **descendant** of a node `x` is a node `y` that is on the path from node `x` to some leaf node.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 1
// **Output:** 3
// **Explanation:** The LCA of nodes 5 and 1 is 3.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 4
// **Output:** 5
// **Explanation:** The LCA of nodes 5 and 4 is 5. A node can be a descendant of itself according to the definition of LCA.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 10
// **Output:** null
// **Explanation:** Node 10 does not exist in the tree, so return null.

// **Constraints:**

// *   The number of nodes in the tree is in the range `[1, 104]`.
// *   `-109 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   `p != q`

// **Follow up:** Can you find the LCA traversing the tree, without checking nodes existence?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft)

//  TreeNode lowest_common_ancestor(TreeNode root, TreeNode p, TreeNode q)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<i32> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, path: &mut Vec<i32>) -> bool {
            if root.is_none() {
                return false;
            }
            let node = root.as_ref().unwrap().borrow();
            path.push(node.val);
            if node.val == v || dfs(&node.left, v, path) || dfs(&node.right, v, path) {
                return true;
            }
            path.pop();
            false
        }
        let mut pp = Vec::new();
        if !dfs(&root, p, &mut pp) {
            return None;
        }

        let mut qp = Vec::new();
        if !dfs(&root, q, &mut qp) {
            return None;
        }
        let mut ans = None;
        let mut i = 0;
        while i < pp.len() && i < qp.len() {
            if pp[i] == qp[i] {
                ans = Some(pp[i]);
            } else {
                break;
            }
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_lowest_common_ancestor_1() {
        assert_eq!(
            Some(3),
            Solution::lowest_common_ancestor(tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 1)
        );
    }
    #[test]
    pub fn test_lowest_common_ancestor_2() {
        assert_eq!(
            Some(5),
            Solution::lowest_common_ancestor(tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 4)
        );
    }
    #[test]
    pub fn test_lowest_common_ancestor_3() {
        assert_eq!(
            None,
            Solution::lowest_common_ancestor(tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], 5, 10)
        );
    }
}
