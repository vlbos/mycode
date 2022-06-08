/*
 * @lc app=leetcode id=1161 lang=rust
 *
 * [1161] Maximum Level Sum of a Binary Tree
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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut maxsum = i32::MIN;
        let mut level = 1;
        let mut q = std::collections::VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        q.push_back(root.clone());
        while !q.is_empty(){
            let n = q.len();
            let mut sum = 0;
            for _ in 0..n{
                let p = q.pop_front().unwrap();
                if let Some(node)=p{
                    sum+=node.borrow().val;
                    if node.borrow().left.is_some(){
                        q.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some(){
                        q.push_back(node.borrow().right.clone());
                    }
                }
            }
            if sum>maxsum{
                ans = level;
                maxsum=sum;
            }
            level+=1;
        }
        ans
    }
}
// @lc code=end

