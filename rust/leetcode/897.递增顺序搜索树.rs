/*
 * @lc app=leetcode.cn id=897 lang=rust
 *
 * [897] 递增顺序搜索树
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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>,mut vals: &mut  Vec<i32>){
             if let Some(n)=root{
                  inorder(&n.borrow().left,&mut vals);
                  vals.push(n.borrow().val);
                  inorder(&n.borrow().right,&mut vals);
            }
        }
        fn newrightnode(mut vals:&mut Vec<i32>)->Option<Rc<RefCell<TreeNode>>>{
             if vals.is_empty(){
               return None;
             }
             let val = vals.pop().unwrap();
           
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().right=newrightnode(vals);
            Some(node)
        }
        if let Some(n)=root.clone(){
                let mut vals = Vec::new();
                inorder(&root,&mut vals);
                vals.reverse();
                let val = vals.pop().unwrap();
                *n.borrow_mut()=TreeNode::new(val);
                n.borrow_mut().right=newrightnode(&mut vals);
        }
        root
    }
}
// @lc code=end

