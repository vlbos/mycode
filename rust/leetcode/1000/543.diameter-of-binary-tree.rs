/*
 * @lc app=leetcode id=543 lang=rust
 *
 * [543] Diameter of Binary Tree
 */

// @lc code=start
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
         fn depth(root: &Option<Rc<RefCell<TreeNode>>>, mut max: &mut i32) -> i32 {
            if let Some(_r) = root {
                let n = _r.borrow();
                let l = depth(&n.left, &mut max);
                let r = depth(&n.right, &mut max);
                *max = (*max).max(l + r + 1);
                return l.max(r) + 1;
            }
            0
        }
        let mut max = 0;
        depth(&root, &mut max);
        max - 1
    }
}
// @lc code=end

