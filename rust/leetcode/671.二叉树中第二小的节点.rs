/*
 * @lc app=leetcode.cn id=671 lang=rust
 *
 * [671] 二叉树中第二小的节点
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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        fn inner_find_second_minimum_value(root: &Option<Rc<RefCell<TreeNode>>>,min:i32)-> i32 {
             if let Some(_n)=root{
             let n =_n.borrow();
             match (&n.left,&n.right){
             (Some(_l),Some(_r))=>{
                     let rv =  inner_find_second_minimum_value(&n.right,min);
                     let lv =  inner_find_second_minimum_value(&n.left,min);
                     if rv>min && lv>min    && rv!=-1 && lv!=-1{
                        return lv.min(rv);
                     }else if rv!=-1 && rv>=min {return rv;}
                    else if lv!=-1 && lv>=min {return lv;}
             },
             _=> if n.val>min {return n.val;},
            }
        }
        -1
        }
        if let Some(_n)=root{
             let n =_n.borrow();
             match (&n.left,&n.right){
             (Some(_l),Some(_r))=>{
                        let rv =  inner_find_second_minimum_value(&n.right,n.val);
                        let lv =  inner_find_second_minimum_value(&n.left,n.val);
                       if rv>n.val && lv>n.val    && rv!=-1 && lv!=-1{
                        return lv.min(rv);
                     }else if rv!=-1 && rv>=n.val {return rv;}
                    else if lv!=-1 && lv>=n.val {return lv;}
             },
             _=>(),
            }
        }
        -1
    }
}
// @lc code=end

