/*
 * @lc app=leetcode.cn id=501 lang=rust
 *
 * [501] 二叉搜索树中的众数
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
         let mut base = 0;
        let mut count = 0;
        let mut max = 0;
         let mut r = Vec::<i32>::new();
         fn update(x:i32,mut base:&mut i32,mut count:&mut i32,mut max:&mut i32,mut r:&mut Vec<i32>){
                if x==*base{
                *count +=1;
                }else{
                    *count = 1;
                    *base =x;
                }
                if *count==*max{
                    r.push(*base);
                }
                else if *count>*max{
                    *max=*count;
                    *r=vec![*base].to_vec();
                }
            };
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>,mut base:&mut i32,mut count:&mut i32,mut max:&mut i32,mut r:&mut Vec<i32>){
            if let Some(_r)=root{
                let n = _r.borrow();
                dfs(&n.left,&mut base,&mut count,&mut max,&mut r);
                update(n.val,&mut base,&mut count,&mut max,&mut r);
                dfs(&n.right,&mut base,&mut count,&mut max,&mut r);
            }
        }
        dfs(&root,&mut base,&mut count,&mut max,&mut r);
        r
    }
}
// @lc code=end

