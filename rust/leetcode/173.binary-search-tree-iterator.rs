/*
 * @lc app=leetcode id=173 lang=rust
 *
 * [173] Binary Search Tree Iterator
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
struct BSTIterator {
   s:Vec<Rc<RefCell<TreeNode>>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut s = Vec::new();
        let mut r = root;
        while let Some(n)=r{
            r= n.borrow().left.clone();
            s.push(n);
        }
        Self{s}
    }
    
    fn next(&mut self) -> i32 {
        let node = self.s.pop().unwrap();
        let val = node.borrow().val;
        let mut r = node.borrow().right.clone();
        while let Some(n)=r{
            r = n.borrow().left.clone();
            self.s.push(n);
        }
        val
    }
    
    fn has_next(&self) -> bool {
        !self.s.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

