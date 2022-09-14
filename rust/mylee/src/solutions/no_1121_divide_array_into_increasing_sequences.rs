// 1121\. Divide Array Into Increasing Sequences
// =============================================

// Given a **non-decreasing** array of positive integers `nums` and an integer `K`,
// find out if this array can be divided into one or more **disjoint increasing subsequences of length at least** `K`.

// **Example 1:**

// **Input:** nums = \[1,2,2,3,3,4,4\], K = 3
// **Output:** true
// **Explanation:**
// The array can be divided into the two subsequences \[1,2,3,4\] and \[2,3,4\] with lengths at least 3 each.

// **Example 2:**

// **Input:** nums = \[5,6,6,7,8\], K = 3
// **Output:** false
// **Explanation:**
// There is no way to divide the array using the conditions required.

// **Note:**

// 1.  `1 <= nums.length <= 10^5`
// 2.  `1 <= K <= nums.length`
// 3.  `1 <= nums[i] <= 10^5`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn can_divide_into_subsequences(nums: Vec<i32>, k: i32) -> bool {
        let (mut cur, mut groups) = (1, 1);
        for num in nums.windows(2) {
            cur = if num[0] < num[1] { 1 } else { cur + 1 };
            groups = groups.max(cur);
        }
        (nums.len() as i32) >= k * groups
    }
}


// // Definition for a binary tree node.
// // #[derive(Debug, PartialEq, Eq)]
// // pub struct TreeNode {
// //   pub val: i32,
// //   pub left: Option<Rc<RefCell<TreeNode>>>,
// //   pub right: Option<Rc<RefCell<TreeNode>>>,
// // }
// //
// // impl TreeNode {
// //   #[inline]
// //   pub fn new(val: i32) -> Self {
// //     TreeNode {
// //       val,
// //       left: None,
// //       right: None
// //     }
// //   }
// // }
// use std::rc::Rc;
// use std::cell::RefCell;
// impl Solution {
//     pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {

//     fn post_order(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut f64) -> (i32, i32) {
//         if let Some(r) = root {
//             let (sum_left, cnt_left) = post_order(&r.borrow().left, ans);
//             let (sum_right, cnt_right) = post_order(&r.borrow().right, ans);
//             let mut sum = r.borrow().val + sum_left + sum_right;
//             let mut cnt = cnt_left + cnt_right + 1;

//             *ans = (*ans).max(sum as f64 / cnt as f64);

//             (sum, cnt)
//         } else {
//             (0, 0)
//         }
//     }
// let mut ans = f64::MIN;

//         post_order(&root, &mut ans);

//         ans
//     }

// }

#[cfg(test)]
mod test {
    use super::*;
// [2,null,1]
// 输出：
// 2.00000
// 预期结果：
// 1.5
    #[test]
    pub fn test_can_divide_into_subsequences_1() {
        assert!(Solution::can_divide_into_subsequences(
            vec![1, 2, 2, 3, 3, 4, 4],
            3
        ));
    }

    #[test]
    pub fn test_can_divide_into_subsequences_2() {
        assert!(!Solution::can_divide_into_subsequences(
            vec![5, 6, 6, 7, 8],
            3
        ));
    }
}
