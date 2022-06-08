/*
 * @lc app=leetcode id=701 lang=rust
 *
 * [701] Insert into a Binary Search Tree
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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
          if let Some(n)=root{
                let mut node = n.borrow_mut();
                if val<node.val{
                     if node.left.is_some(){
                         node.left=Self::insert_into_bst(node.left.clone(),val);
                     }else{
                        node.left=Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    }
                }else {
                     if node.right.is_some(){
                         node.right=Self::insert_into_bst(node.right.clone(),val);
                     }else{
                        node.right=Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    }
                }
                return Some(n.clone());
          }
          Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}
// @lc code=end

