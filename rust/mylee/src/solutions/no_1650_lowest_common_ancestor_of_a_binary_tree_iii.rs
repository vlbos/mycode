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

// According to the **[definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor)**:
//  "The lowest common ancestor of two nodes p and q in a tree T is the lowest node that has both p and q as descendants
// (where we allow **a node to be a descendant of itself**)."

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)

// **Input:** root = \[3,5,1,6,2,0,8,null,null,7,4\], p = 5, q = 1
// **Output:** 3
// **Explanation:5** The LCA of nodes 5 and 1 is 3.

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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, parent: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            parent,
        }
    }
}

pub fn to_ptree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap(), None))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(
                v,
                Some(parent.clone()),
            ))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(
                    v,
                    Some(parent.clone()),
                ))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! ptree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_ptree(vec)
        }
    };
    ($($e:expr,)*) => {(ptree![$($e),*])};
}

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut a = p.clone();
        let mut b = q.clone();
        while a.as_ref().unwrap().borrow().val != b.as_ref().unwrap().borrow().val {
            a = if a.as_ref().unwrap().borrow().parent.is_some() {
                a.as_ref().unwrap().borrow().parent.clone()
            } else {
                q.clone()
            };
            b = if b.as_ref().unwrap().borrow().parent.is_some() {
                b.as_ref().unwrap().borrow().parent.clone()
            } else {
                p.clone()
            };
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        let t = ptree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        assert_eq!(
            dfs(&t, 3),
            Solution::lowest_common_ancestor(t.clone(), dfs(&t, 5), dfs(&t, 1))
        );
    }
    #[test]
    pub fn test_lowest_common_ancestor_2() {
        let t = ptree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let n5 = dfs(&t, 5);
        assert_eq!(
            n5.clone(),
            Solution::lowest_common_ancestor(t.clone(), n5, dfs(&t, 4))
        );
    }
    #[test]
    pub fn test_lowest_common_ancestor_3() {
        let t = ptree![1, 2];
        let n1 = dfs(&t, 1);
        assert_eq!(
            n1.clone(),
            Solution::lowest_common_ancestor(t.clone(), n1, dfs(&t, 2))
        );
    }
}
