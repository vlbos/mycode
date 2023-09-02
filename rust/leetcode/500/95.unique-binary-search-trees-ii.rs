/*
 * @lc app=leetcode id=95 lang=rust
 *
 * [95] Unique Binary Search Trees II
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if l == r {
                return vec![Some(Rc::new(RefCell::new(TreeNode::new(l))))];
            }
            let mut res = vec![];
            for pivot in l..=r {
                let l_trees = if pivot == l {
                    vec![None]
                } else {
                    helper(l, pivot - 1)
                };
                let r_trees = if pivot == r {
                    vec![None]
                } else {
                    helper(pivot + 1, r)
                };
                for i in 0..l_trees.len() {
                    for j in 0..r_trees.len() {
                        let tree = TreeNode {
                            val: pivot,
                            left: l_trees[i].clone(),
                            right: r_trees[j].clone(),
                        };
                        res.push(Some(Rc::new(RefCell::new(tree))));
                    }
                }
            }
            res
        }
        helper(1, n)
    }
}
// @lc code=end
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut dp=vec![vec![]; n as usize+1];
        dp[0].push(None);
        fn clone(node:&Option<Rc<RefCell<TreeNode>>>,offset:i32)->Option<Rc<RefCell<TreeNode>>>{
            if node.is_none(){
                return None
            }
            Some(Rc::new(RefCell::new(TreeNode{val:node.as_ref().unwrap().borrow().val+offset,left:clone(&node.as_ref().unwrap().borrow().left,offset),right:clone(&node.as_ref().unwrap().borrow().right,offset)})))
        }
        for len in 1..=n{
            for root in 1..=len{
                let (l,r)=(root-1,len-root);
                for left in &dp[l as usize].clone(){
                    for right in &dp[r as usize].clone(){
                        let mut node=TreeNode::new(root);
                        dp[len as usize].push(Some(Rc::new(RefCell::new(TreeNode{val:root,left:left.clone(),right:clone(right,root)}))));
                    }
                }

            }
        }
        dp[n as usize].clone()
    }
}