// 1602\. Find Nearest Right Node in Binary Tree
// =============================================

// Given the `root` of a binary tree and a node `u` in the tree,
// return _the **nearest** node on the **same level** that is to the **right** of_ `u`_,
// or return_ `null` _if_ `u` _is the rightmost node in its level_.

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2020/09/24/p3.png)

// **Input:** root = \[1,2,3,null,4,5,6\], u = 4
// **Output:** 5
// **Explanation:** The nearest node on the same level to the right of node 4 is node 5.

// **Example 2:**

// **![](https://assets.leetcode.com/uploads/2020/09/23/p2.png)**

// **Input:** root = \[3,null,4,2\], u = 2
// **Output:** null
// **Explanation:** There are no nodes to the right of 2.

// **Example 3:**

// **Input:** root = \[1\], u = 1
// **Output:** null

// **Example 4:**

// **Input:** root = \[3,4,2,null,null,null,1\], u = 4
// **Output:** 2

// **Constraints:**

// *   The number of nodes in the tree is in the range `[1, 105]`.
// *   `1 <= Node.val <= 105`
// *   All values in the tree are **distinct**.
// *   `u` is a node in the binary tree rooted at `root`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//  find_nearest_right_node_in_binary_tree
//  TreeNode find_nearest_right_node(TreeNode root, TreeNode u) {

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_nearest_right_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        u: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q = std::collections::VecDeque::from([root.clone()]);
        while !q.is_empty() {
            let n = q.len();
            for i in 0..n {
                let v = q.pop_front().unwrap();
                if v.as_ref().unwrap().borrow().val == u.as_ref().unwrap().borrow().val {
                    return if i + 1 < n {
                        q.pop_front().unwrap()
                    } else {
                        None
                    };
                }
                if v.as_ref().unwrap().borrow().left.is_some() {
                    q.push_back(v.as_ref().unwrap().borrow().left.clone());
                }
                if v.as_ref().unwrap().borrow().right.is_some() {
                    q.push_back(v.as_ref().unwrap().borrow().right.clone());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    use std::collections::HashSet;
    #[test]
    pub fn test_find_nearest_right_node_1() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            Solution::find_nearest_right_node(
                tree![1, 2, 3, null, 4, 5, 6],
                Some(Rc::new(RefCell::new(TreeNode::new(4))))
            )
        );
    }
    #[test]
    pub fn test_find_nearest_right_node_2() {
        assert_eq!(
            None,
            Solution::find_nearest_right_node(
                tree![3, null, 4, 2],
                Some(Rc::new(RefCell::new(TreeNode::new(2))))
            )
        );
    }
    #[test]
    pub fn test_find_nearest_right_node_3() {
        assert_eq!(
            None,
            Solution::find_nearest_right_node(
                tree![1],
                Some(Rc::new(RefCell::new(TreeNode::new(1))))
            )
        );
    }
    #[test]
    pub fn test_find_nearest_right_node_4() {
        assert_eq!(
            tree![2, null, 1],
            Solution::find_nearest_right_node(
                tree![3, 4, 2, null, null, null, 1],
                Some(Rc::new(RefCell::new(TreeNode::new(4))))
            )
        );
    }
}
