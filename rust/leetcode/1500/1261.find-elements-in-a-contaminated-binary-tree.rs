/*
 * @lc app=leetcode id=1261 lang=rust
 *
 * [1261] Find Elements in a Contaminated Binary Tree
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
struct FindElements {
root: Option<Rc<RefCell<TreeNode>>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self{root}
    }
    
    fn find(&self, target: i32) -> bool {
         fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,val:i32,target:i32)->bool{
            if let Some(n)=root{
                if val==target{
                return true;
                }
                return dfs(&n.borrow().left,val*2+1,target)|| dfs(&n.borrow().right,val*2+2,target);
            }
            false
         }
         dfs(&self.root,0,target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
// @lc code=end

