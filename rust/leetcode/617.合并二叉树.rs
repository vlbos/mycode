/*
 * @lc app=leetcode.cn id=617 lang=rust
 *
 * [617] 合并二叉树
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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inner_merge_trees(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
            match (root1,root2){
                (Some(n1),Some(n2))=>{
                      let (_n1,_n2)=(n1.borrow(),n2.borrow());
                      let mut root = TreeNode::new(_n1.val+_n2.val);
                      root.left = inner_merge_trees(&_n1.left,&_n2.left);
                      root.right = inner_merge_trees(&_n1.right,&_n2.right);
                      Some(Rc::new(RefCell::new(root)))
                },
                (Some(n1),None)=>Some(n1.clone()),
                (None,Some(n2))=>Some(n2.clone()),
                (None,None)=>None,
            }
        } 
        inner_merge_trees(&root1,&root2)
    }
}
// @lc code=end

