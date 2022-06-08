/*
 * @lc app=leetcode id=515 lang=rust
 *
 * [515] Find Largest Value in Each Tree Row
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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn pre_order(root: &Option<Rc<RefCell<TreeNode>>>,level:usize,v:&mut Vec<i32>){
            if let Some(n)=root{
                if v.len()<=level{
                    v.push(n.borrow().val);
                 }else if n.borrow().val>v[level]{
                        v[level]= n.borrow().val;
                 }
                pre_order(&n.borrow().left,level+1,v);
                pre_order(&n.borrow().right,level+1,v);
            }
        }
        let mut ans =Vec::new();
        pre_order(&root,0,&mut ans);
        ans
    }
}
// @lc code=end

