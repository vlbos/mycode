/*
 * @lc app=leetcode id=1022 lang=rust
 *
 * [1022] Sum of Root To Leaf Binary Numbers
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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(root: &Option<Rc<RefCell<TreeNode>>>,path:i32,mut sum:&mut i32){
            if let Some(ref n)=&root{
                let newpath = path*2+n.borrow().val;
                if n.borrow().left.is_none()&& n.borrow().right.is_none(){
                    *sum+=newpath;
                    return;
                }
                depth(&n.borrow().left,newpath,&mut sum);
                depth(&n.borrow().right,newpath,&mut sum);
            }
        }
        let mut sum = 0;
        depth(&root,0,&mut sum);
        sum
    }
}
// @lc code=end

