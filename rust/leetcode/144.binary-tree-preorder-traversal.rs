/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        let mut s = Vec::<Rc<RefCell<TreeNode>>>::new();
        let mut v = Vec::<i32>::new();
        let mut n = root.clone();
        while n.is_some() || !s.is_empty() {
            while let Some(_n) = n {
                v.push(_n.borrow().val);
                s.push(_n.clone());
                n = _n.borrow().left.clone();
                if n.is_none() {
                    break;
                }
            }
            n = s.pop();
            if let Some(_n) = n {
                n = _n.borrow().right.clone();
            }
        }
        v
    }
}
// @lc code=end

