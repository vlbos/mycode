/*
 * @lc app=leetcode id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn in_order(root: &Option<Rc<RefCell<TreeNode>>>, k: usize, seq: &mut Vec<i32>) {
            if seq.len() == k {
                return;
            }
            if let Some(n) = root {
                in_order(&n.borrow().left, k, seq);
                if seq.len() == k {
                    return;
                }
                seq.push(n.borrow().val);
                if seq.len() == k {
                    return;
                }
                in_order(&n.borrow().right, k, seq);
            }
        }
        let mut seq = Vec::new();
        in_order(&root, k as usize, &mut seq);
        // seq[k as usize - 1]
        *seq.last().unwrap()
    }
}
// @lc code=end
