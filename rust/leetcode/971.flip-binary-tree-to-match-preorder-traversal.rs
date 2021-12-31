/*
 * @lc app=leetcode id=971 lang=rust
 *
 * [971] Flip Binary Tree To Match Preorder Traversal
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
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
            fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,voyage: &Vec<i32>,ans:&mut Vec<i32>,index:&mut usize){
                if let Some(node)=root{
                    if node.borrow().val!=voyage[*index]{
                        ans.clear();
                        ans.push(-1);
                        return;
                    }
                    *index+=1;
                    if *index<voyage.len() && node.borrow().left.is_some() && node.as_ref().borrow().left.as_ref().unwrap().borrow().val!=voyage[*index]{
                        ans.push(node.borrow().val);
                        dfs(&node.borrow().right,voyage,ans,index);
                        dfs(&node.borrow().left,voyage,ans,index);
                    }else{
                        dfs(&node.borrow().left,voyage,ans,index);
                        dfs(&node.borrow().right,voyage,ans,index);
                    }
                }
            }
            dfs(&root,&voyage,&mut ans,&mut 0);
            if *ans.first().unwrap_or(&0)==-1{
                ans.clear();
                ans.push(-1);
            }
            ans
    }
}
// @lc code=end

