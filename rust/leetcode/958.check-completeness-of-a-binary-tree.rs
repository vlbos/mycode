/*
 * @lc app=leetcode id=958 lang=rust
 *
 * [958] Check Completeness of a Binary Tree
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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
         let mut nodes = Vec::new();
        let mut i = 0;
        nodes.push((root.clone(),1));
        while i<nodes.len(){
                let (node,position)=nodes.get(i).unwrap();
                if let Some(n)=&node{
                   let l = n.borrow().left.clone();
                   let r = n.borrow().right.clone();
                    let pos = position.clone();
                    nodes.push((l,2*pos));
                    nodes.push((r,2*pos+1));
                }
            i+=1;
        }
        nodes.last().unwrap().1==nodes.len()
    }
}
// @lc code=end

