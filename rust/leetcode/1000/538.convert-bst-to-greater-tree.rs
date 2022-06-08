/*
 * @lc app=leetcode id=538 lang=rust
 *
 * [538] Convert BST to Greater Tree
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn order(root: Option<&RefCell<TreeNode>>, sum: &mut i32) {
            if let Some(mut n) = root.map(RefCell::borrow_mut) {
                order(n.right.as_deref(), sum);
                *sum += n.val;
                n.val = *sum;
                order(n.left.as_deref(), sum);
            }
        }
        order(root.as_deref(), &mut 0);
        root
    }
}
// @lc code=end
