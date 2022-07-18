// 270\. Closest Binary Search Tree Value
// ======================================

// Given a non-empty binary search tree and a target value, find the value in the BST that is closest to the target.

// **Note:**

// *   Given target value is a floating point.
// *   You are guaranteed to have only one unique value in the BST that is closest to the target.

// **Example:**

// **Input:** root = \[4,2,5,1,3\], target = 3.714286

//     4
//    / \\
//   2   5
//  / \\
// 1   3

// **Output:** 4

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Snapchat](https://leetcode.ca/tags/#Snapchat)
use super::util::tree::{to_tree, TreeNode};
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        // Solution::closest_value_rec(&root, target).unwrap()
        let node = root.as_ref().unwrap().borrow();
        let val = node.val;
        let t = if target < val as f64 {
            node.left.clone()
        } else {
            node.right.clone()
        };
        if t.is_none() {
            return val;
        }
        let cval = Self::closest_value(t, target);
        if (val as f64 - target).abs() < (cval as f64 - target).abs() {
            val
        } else {
            cval
        }
    }

    // fn closest_value_rec(root: &Option<Rc<RefCell<TreeNode>>>, target: f64) -> Option<i32> {
    //     match root {
    //         Some(node_ref) => {
    //             let node_b = node_ref.borrow();
    //             let mut another_val: i32 = i32::min_value();
    //             let mut has_another = false;
    //             if target < (node_b.val as f64) {
    //                 if let Some(left_max) = Solution::closest_value_rec(&node_b.left, target) {
    //                     another_val = left_max;
    //                     has_another = true;
    //                 }
    //             } else if target > (node_b.val as f64) {
    //                 if let Some(right_min) = Solution::closest_value_rec(&node_b.right, target) {
    //                     another_val = right_min;
    //                     has_another = true;
    //                 }
    //             }
    //             if has_another {
    //                 if f64::abs((another_val as f64) - target)
    //                     < f64::abs((node_b.val as f64) - target)
    //                 {
    //                     Some(another_val)
    //                 } else {
    //                     Some(node_b.val)
    //                 }
    //             } else {
    //                 Some(node_b.val)
    //             }
    //         }
    //         None => None,
    //     }
    // }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;

    #[test]
    fn test_closest_value_rec() {
        assert_eq!(
            Solution::closest_value(tree![4, 5, 2, null, null, 3, 1], 3.714286),
            4
        );
    }
}
