/*
 * @lc app=leetcode id=662 lang=rust
 *
 * [662] Maximum Width of Binary Tree
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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,depth:usize,pos:i32,left:&mut Vec<i32>,ans:&mut i32){
                if let Some(n)=root{
                    if depth>=left.len(){
                        left.push(pos);
                    }
                    *ans = (*ans).max(pos-left[depth]+1);
                    dfs(&n.borrow().left,depth+1,pos*2,left,ans);
                    dfs(&n.borrow().right,depth+1,pos*2+1,left,ans);
                }
        }
        let mut left=Vec::new();
        let mut ans = 0;
        dfs(&root,0,0,&mut left,&mut ans);
        ans
    }
}
// @lc code=end

