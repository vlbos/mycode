/*
 * @lc app=leetcode id=1382 lang=rust
 *
 * [1382] Balance a Binary Search Tree
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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn in_order(root: &Option<Rc<RefCell<TreeNode>>>,ans:&mut Vec<i32>){
            if let Some(n)=root{
                in_order(&n.borrow().left,ans);
                ans.push(n.borrow().val);
                in_order(&n.borrow().right,ans);
            }
        }
        let mut ans = Vec::new();
        in_order(&root,&mut ans);
        fn build_tree(a:& Vec<i32>,l:i32,r:i32)->Option<Rc<RefCell<TreeNode>>> {
            if l>r{
            return None;
            }
            let mid = (l+r)/2;
            let mut node = TreeNode::new(a[mid as usize]);
            if mid-1>=l{
                node.left = build_tree(a,l,mid-1);
            }
            if mid+1<=r{
                node.right = build_tree(a,mid+1,r);
            }
            Some(Rc::new(RefCell::new(node)))
        }
        build_tree(&ans,0,ans.len() as i32-1)
    }
}
// @lc code=end

