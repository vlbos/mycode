/*
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * [145] 二叉树的后序遍历
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut v = Vec::<i32>::new();
        // if let Some(n)=root{
        //     let mut l = Self::postorder_traversal(n.borrow().left.clone());
        //     if !l.is_empty(){
        //         v.append(&mut l);
        //     }
        //     let mut r = Self::postorder_traversal(n.borrow().right.clone());
        //     if !r.is_empty(){
        //         v.append(&mut r);
        //     }
        //     v.push(n.borrow().val);
        // }
        // v
        let mut v = Vec::<i32>::new();
        let mut s = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        s.push(root);
        while let Some(Some(n)) = s.pop() {
            if let Some(l) = &n.borrow().left {
                s.push(Some(Rc::clone(&l)));
            }
            if let Some(r) = &n.borrow().right {
                s.push(Some(Rc::clone(&r)));
            }
            v.insert(0, n.borrow().val);
        }
        v
    }
}
// @lc code=end
