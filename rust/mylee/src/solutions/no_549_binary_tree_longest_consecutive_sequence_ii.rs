// 549\. Binary Tree Longest Consecutive Sequence II
// =================================================

// Given a binary tree, you need to find the length of Longest Consecutive Path in Binary Tree.

// Especially, this path can be either increasing or decreasing. For example, \[1,2,3,4\] and \[4,3,2,1\] are both considered valid,
// but the path \[1,2,4,3\] is not valid. On the other hand, the path can be in the child-Parent-child order, where not necessarily be parent-child order.

// **Example 1:**

// **Input:**
//         1
//        / \\
//       2   3
// **Output:** 2
// **Explanation:** The longest consecutive path is \[1, 2\] or \[2, 1\].

// **Example 2:**

// **Input:**
//         2
//        / \\
//       1   3
// **Output:** 3
// **Explanation:** The longest consecutive path is \[1, 2, 3\] or \[3, 2, 1\].

// **Note:** All the values of tree nodes are in the range of \[-1e7, 1e7\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google)

use crate::solutions::util::tree::TreeNode;

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub  struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn   new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

// pub  struct Data {
//     val: i32,
//     increase: Option<usize>,
//     decease: Option<usize>,
// }

impl Solution {
    pub fn   longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let mut max = 0usize;
        // Solution::longest_consecutive_recursive(root, &mut max);
        // max as i32
        let mut ans = 0;
       pub fn  solve(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
            let (mut inc, mut dec) = (0, 0);
            if let Some(node) = root {
                let node = node.borrow();
                for child in [&node.left, &node.right] {
                    if child.is_none() {
                        continue;
                    }
                    let (cinc, cdec) = solve(child, ans);
                    if child.as_ref().unwrap().borrow().val == node.val - 1 {
                        dec = dec.max(cdec);
                    } else if child.as_ref().unwrap().borrow().val == node.val + 1 {
                        inc = inc.max(cinc);
                    }
                }
                *ans = (*ans).max(inc + dec + 1);
            }
            (inc + 1, dec + 1)
        }
        solve(&root, &mut ans);
        ans
    }

    //pub fn  longest_consecutive_recursive(
    //     curr: Option<Rc<RefCell<TreeNode>>>,
    //     mut max_len: &mut usize,
    // ) -> Data {
    //     match curr {
    //         Some(curr_ref) => {
    //             let mut curr_node = curr_ref.borrow_mut();
    //             let val = curr_node.val;
    //             let left_res =
    //                 Solution::longest_consecutive_recursive(curr_node.left.take(), &mut max_len);
    //             let right_res =
    //                 Solution::longest_consecutive_recursive(curr_node.right.take(), &mut max_len);
    //             let increase_left = if left_res.val + 1 == val {
    //                 left_res.increase.unwrap_or(0)
    //             } else {
    //                 0
    //             };
    //             let decrease_left = if left_res.val == 1 + val {
    //                 left_res.decease.unwrap_or(0)
    //             } else {
    //                 0
    //             };
    //             let increase_right = if right_res.val + 1 == val {
    //                 right_res.increase.unwrap_or(0)
    //             } else {
    //                 0
    //             };
    //             let decrease_right = if right_res.val == 1 + val {
    //                 right_res.decease.unwrap_or(0)
    //             } else {
    //                 0
    //             };
    //             *max_len = usize::max(
    //                 *max_len,
    //                 usize::max(
    //                     increase_left + decrease_right,
    //                     decrease_left + increase_right,
    //                 ) + 1,
    //             );
    //             Data {
    //                 val,
    //                 increase: Some(usize::max(increase_left, increase_right) + 1),
    //                 decease: Some(usize::max(decrease_left, decrease_right) + 1),
    //             }
    //         }
    //         None => Data {
    //             val: 0,
    //             increase: None,
    //             decease: None,
    //         },
    //     }
    // }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
   pub fn  test_longest_consecutive_1() {
        let tree = tree![1, 2, 3];
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }

    #[test]
   pub fn  test_longest_consecutive_2() {
        let tree = tree![2, 1, 3];
        assert_eq!(Solution::longest_consecutive(tree), 3);
    }

    #[test]
   pub fn  test_longest_consecutive_3() {
        let tree = tree![1, 2, 4, 3];
        assert_eq!(Solution::longest_consecutive(tree), 3);
    }

    #[test]
   pub fn  test_longest_consecutive_4() {
        let tree = tree![3, 1, 2];
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }
}
