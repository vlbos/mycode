// 776\. Split BST
// ===============

// Given a Binary Search Tree (BST) with root node `root`, and a target value `V`,
// split the tree into two subtrees where one subtree has nodes that are all smaller or equal to the target value,
// while the other subtree has all nodes that are greater than the target value.
// It's not necessarily the case that the tree contains a node with value `V`.

// Additionally, most of the structure of the original tree should remain.
//  Formally, for any child C with parent P in the original tree, if they are both in the same subtree after the split,
// then node C should still have the parent P.

// You should output the root TreeNode of both subtrees after splitting, in any order.

// **Example 1:**

// **Input:** root = \[4,2,6,1,3,5,7\], V = 2
// **Output:** \[\[2,1\],\[4,3,6,null,null,5,7\]\]
// **Explanation:**
// Note that root, output\[0\], and output\[1\] are TreeNode objects, not arrays.

// The given tree \[4,2,6,1,3,5,7\] is represented by the following diagram:

//           4
//         /   \\
//       2      6
//      / \\    / \\
//     1   3  5   7

// while the diagrams for the outputs are:

//           4
//         /   \\
//       3      6      and    2
//             / \\           /
//            5   7         1

// **Note:**

// 1.  The size of the BST will not exceed `50`.
// 2.  The BST is always valid and each node's value is different.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Coupang](https://leetcode.ca/tags/#Coupang) [Google](https://leetcode.ca/tags/#Google)

// Definition for a binary tree node
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNodev {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

// pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
//     use std::collections::VecDeque;
//     let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
//     let mut queue = VecDeque::new();
//     queue.push_back(head.as_ref().unwrap().clone());

//     for children in vec[1..].chunks(2) {
//         let parent = queue.pop_front().unwrap();
//         if let Some(v) = children[0] {
//             parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
//             queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
//         }
//         if children.len() > 1 {
//             if let Some(v) = children[1] {
//                 parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
//                 queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
//             }
//         }
//     }
//     head
// }
use super::util::tree::TreeNode;
#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
#[allow(dead_code)]
    pub fn split_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        v: i32,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = vec![None, None];
        if root.is_none() {
            return ans;
        }
        if root.as_ref().unwrap().borrow().val <= v {
            ans = Self::split_bst(root.as_ref().unwrap().borrow().right.clone(), v);
            root.as_mut().unwrap().borrow_mut().right = ans[0].clone();
            ans[0] = root;
        } else {
            ans = Self::split_bst(root.as_ref().unwrap().borrow().left.clone(), v);
            root.as_mut().unwrap().borrow_mut().left = ans[1].clone();
            ans[1] = root;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_split_bst_1() {
        assert_eq!(
            vec![tree![2, 1], tree![4, 3, 6, null, null, 5, 7]],
            Solution::split_bst(tree![4, 2, 6, 1, 3, 5, 7], 2)
        );
    }
}
