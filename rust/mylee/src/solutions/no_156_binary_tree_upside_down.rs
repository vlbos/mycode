// 156	Binary Tree Upside Down

//  Given a binary tree where all the right nodes are
//      either leaf nodes with a sibling (a left node that shares the same parent node)
//      or empty,
//  flip it upside down and turn it into a tree where the original right nodes turned into left leaf nodes.

//  Return the new root.

//  Example:

//  Input: [1,2,3,4,5]

//     1
//    / \
//   2   3
//  / \
// 4   5

//  Output: return the root of the binary tree [4,5,2,#,#,3,1]

//     4
//    / \
//   5   2
//  / \
// 3   1

//  Clarification:

//  Confused what [4,5,2,#,#,3,1] means? Read more below on how binary tree is serialized on OJ.

//  The serialization of a binary tree follows a level order traversal,
//  where '#' signifies a path terminator where no node exists below.

//  Here's an example:

//     1
//    / \
//   2   3
//  /
// 4
//  \
//   5

//  The above binary tree is serialized as [1,2,3,#,#,4,#,#,5].

//  @tag-tree
pub struct Solution {}
use super::util::tree::{to_tree, TreeNode};
// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
//  @lc app=leetcode id=156 lang=rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn upside_down_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn right_sibling(
            root: Option<Rc<RefCell<TreeNode>>>,
            parent: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match &root {
                Some(root_ref) => {
                    let res = {
                        let (root_right, root_left) = {
                            let mut root_mb = root_ref.borrow_mut();
                            let root_right = root_mb.right.take();
                            let root_left = root_mb.left.take();
                            right_sibling(root_right.clone(), None, None);
                            (root_right, root_left)
                        };
                        if root_left.is_some() {
                            right_sibling(root_left, root.clone(), root_right)
                        } else {
                            root.clone()
                        }
                    };
                    {
                        let mut left_mut = root_ref.borrow_mut();
                        left_mut.left = right;
                        left_mut.right = parent;
                    }
                    res
                }
                None => None,
            }
        }
        right_sibling(root, None, None)
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;
    #[test]
    fn test_binary_tree_upside_down_basic() {
        assert_eq!(Solution::upside_down_binary_tree(tree![1,2,3,4,5]), tree![4,5,2,null,null,3,1]);

        assert_eq!(Solution::upside_down_binary_tree(tree![4,5,2,null,null,3,1]), tree![5,2,4]);
    }
}
