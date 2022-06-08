/*
 * @lc app=leetcode id=1609 lang=rust
 *
 * [1609] Even Odd Tree
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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut q = std::collections::VecDeque::new();
        let mut level = 0;
        q.push_back(root.clone());
        while !q.is_empty() {
            let mut p = if level % 2 == 0 { i32::MIN } else { i32::MAX };
            let n = q.len();
            for _ in 0..n {
                if let Some(qq) = q.pop_front() {
                    if let Some(nn) = qq {
                        let node = nn.borrow();
                        let c = node.val;
                        if node.left.is_some() {
                            q.push_back(node.left.clone());
                        }
                        if node.right.is_some() {
                            q.push_back(node.right.clone());
                        }
                        if (level % 2 == 0 && (c <= p || c % 2 == 0))
                            || (level % 2 > 0 && (c >= p || c % 2 > 0))
                        {
                            return false;
                        }
                        p = c;
                    }
                }
            }
            level += 1;
        }
        true
    }
}
// @lc code=end
