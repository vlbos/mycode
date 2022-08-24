// 1676\. Lowest Common Ancestor of a Binary Tree IV
// =================================================

// Given the `root` of a binary tree and an array of `TreeNode` objects `nodes`,
// return _the lowest common ancestor (LCA) of **all the nodes** in_ `nodes`.
//  All the nodes will exist in the tree, and all values of the tree's nodes are **unique**.

// Extending the **[definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor)**:
// "The lowest common ancestor of `n` nodes `p1`, `p2`, ..., `pn` in a binary tree `T` is the lowest node that has every `pi` as a **descendant**
// (where we allow **a node to be a descendant of itself**) for every valid `i`".
//  A **descendant** of a node `x` is a node `y` that is on the path from node `x` to some leaf node.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], nodes = \[4,7\]
// **Output:** 2
// **Explanation:** The lowest common ancestor of nodes 4 and 7 is node 2.

// **Example 2:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], nodes = \[1\]
// **Output:** 1
// **Explanation:** The lowest common ancestor of a single node is the node itself.

// **Example 3:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], nodes = \[7,6,2,4\]
// **Output:** 5
// **Explanation:** The lowest common ancestor of the nodes 7, 6, 2, and 4 is node 5.

// **Example 4:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], nodes = \[0,1,2,3,4,5,6,7,8\]
// **Output:** 3
// **Explanation:** The lowest common ancestor of all the nodes is the root node.

// **Constraints:**

// *   The number of nodes in the tree is in the range `[1, 104]`.
// *   `-109 <= Node.val <= 109`
// *   All `Node.val` are **unique**.
// *   All `nodes[i]` will exist in the tree.
// *   All `nodes[i]` are distinct.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon)

// TreeNode lowestCommonAncestor(TreeNode root, TreeNode[] nodes)

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        nodes: Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashSet;
        let vs: HashSet<i32> = nodes
            .iter()
            .map(|x| x.as_ref().unwrap().borrow().val)
            .collect();
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            vs: &HashSet<i32>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if root.is_none() || vs.contains(&root.as_ref().unwrap().borrow().val) {
                return root.clone();
            }
            let node = root.as_ref().unwrap().borrow();
            let l = dfs(&node.left, vs);
            let r = dfs(&node.right, vs);
            if l.is_none() {
                return r;
            }
            if r.is_none() {
                return l;
            }
            root.clone()
        }

        dfs(&root, &vs)
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
    pub fn test_lowest_common_ancestor_1() {
        let t = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        assert_eq!(
            dfs(&t, 2),
            Solution::lowest_common_ancestor(t.clone(), vec![dfs(&t, 4), dfs(&t, 7)])
        );
    }
    #[test]
    pub fn test_lowest_common_ancestor_2() {
        let t = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let n1 = dfs(&t, 1);
        assert_eq!(
            n1.clone(),
            Solution::lowest_common_ancestor(t.clone(), vec![n1])
        );
    }
    #[test]
    pub fn test_lowest_common_ancestor_3() {
        let t = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> =
            [7, 6, 2, 4].into_iter().map(|i| dfs(&t, i)).collect();
        assert_eq!(
            dfs(&t, 5),
            Solution::lowest_common_ancestor(t.clone(), nodes)
        );
    }

    #[test]
    pub fn test_lowest_common_ancestor_4() {
        let t = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = [0, 1, 2, 3, 4, 5, 6, 7, 8]
            .into_iter()
            .map(|i| dfs(&t, i))
            .collect();
        assert_eq!(
            dfs(&t, 3),
            Solution::lowest_common_ancestor(t.clone(), nodes)
        );
    }
}
