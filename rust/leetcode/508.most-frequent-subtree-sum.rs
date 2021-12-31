/*
 * @lc app=leetcode id=508 lang=rust
 *
 * [508] Most Frequent Subtree Sum
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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        fn  post_order(root: &Option<Rc<RefCell<TreeNode>>>,m:&mut std::collections::HashMap<i32,i32>)->Option<i32>{
            if let Some(n)=root{
                let l = post_order(&n.borrow().left,m);
                let r = post_order(&n.borrow().right,m);
                let mut v = n.borrow().val;
                if let Some(l)=l{
                v+=l;
                }
                if let Some(r)=r{
                v+=r;
                }
                *m.entry(v).or_insert(0)+=1;
                
                return Some(v);
            }
            None
        }
        post_order(&root,&mut m);
        let max = *m.iter().max_by_key(|&(k,v)| v).unwrap().1;
        m.iter().filter(|&(k,v)|*v==max).map(|(k,v)|*k).collect::<Vec<i32>>()
    }
}
// @lc code=end

