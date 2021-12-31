/*
 * @lc app=leetcode id=951 lang=rust
 *
 * [951] Flip Equivalent Binary Trees
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
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (&root1,&root2){
        (None,None)=>true,
        (Some(r1),Some(r2))=>{
            if r1.borrow().val==r2.borrow().val{
                Self::flip_equiv(r1.borrow().left.clone(),r2.borrow().left.clone()) && Self::flip_equiv(r1.borrow().right.clone(),r2.borrow().right.clone())||
                Self::flip_equiv(r1.borrow().left.clone(),r2.borrow().right.clone()) && Self::flip_equiv(r1.borrow().right.clone(),r2.borrow().left.clone())
            }else{
            false
            }
        },
        _=>false,
        }
    }
}
// @lc code=end

