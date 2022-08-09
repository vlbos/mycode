// 272\. Closest Binary Search Tree Value II
// =========================================

// Given a non-empty binary search tree and a target value, find _k_ values in the BST that are closest to the target.

// **Note:**

// *   Given target value is a floating point.
// *   You may assume _k_ is always valid, that is: _k_ ≤ total nodes.
// *   You are guaranteed to have only one unique set of _k_ values in the BST that are closest to the target.

// **Example:**

// **Input:** root = \[4,2,5,1,3\], target = 3.714286, and _k_ = 2

//     4
//    / \\
//   2   5
//  / \\
// 1   3

// **Output:** \[4,3\]

// **Follow up:**
// Assume that the BST is balanced, could you solve it in less than _O_(_n_) runtime (where _n_ \= total nodes)?

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [ForUsAll](https://leetcode.ca/tags/#ForUsAll) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn)
use super::util::tree::{ TreeNode};
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// enum NodeOrVal {
//     Node(Option<Rc<RefCell<TreeNode>>>),
//     Val(i32),
// }

// #[derive(PartialEq, PartialOrd)]
// struct NonNan(f64);

// impl Eq for NonNan {}

// impl Ord for NonNan {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

impl Solution {
    pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
        // let sorted = Solution::inorder_traversal(root);
        // Solution::find_closest(&sorted, target, k as isize)
        let mut ans = Vec::new();
        fn in_order(
            root: &Option<Rc<RefCell<TreeNode>>>,
            target: f64,
            reverse: bool,
            stack: &mut Vec<i32>,
        ) {
            if let Some(node) = root {
                let root = node.borrow();
                in_order(
                    if reverse { &root.right } else { &root.left },
                    target,
                    reverse,
                    stack,
                );
                let val = root.val as f64;
                if (reverse && val <= target) || (!reverse && val > target) {
                    return;
                }
                stack.push(root.val);
                in_order(
                    if reverse { &root.left } else { &root.right },
                    target,
                    reverse,
                    stack,
                );
            }
        }
        let (mut pre, mut suc) = (Vec::new(), Vec::new());
        in_order(&root, target, false, &mut pre);
        in_order(&root, target, true, &mut suc);
        let mut k = k;
        while k > 0 {
            ans.push(if pre.is_empty() {
                pre.pop().unwrap()
            } else if suc.is_empty() {
                suc.pop().unwrap()
            } else if (*pre.last().unwrap() as f64 - target).abs()
                < (*suc.last().unwrap() as f64 - target).abs()
            {
                pre.pop().unwrap()
            } else {
                suc.pop().unwrap()
            });
            k -= 1;
        }
        ans
    }

    // pub fn find_closest(arr: &[i32], target: f64, k: isize) -> Vec<i32> {
    //     if (arr.len() as isize) <= k {
    //         return arr.iter().cloned().collect();
    //     }
    //     let mut begin = 0isize;
    //     let mut end = arr.len() as isize;
    //     let mut heap = BinaryHeap::<(NonNan, i32)>::new();
    //     while begin + 1 < end {
    //         let mid = (begin + end) / 2;
    //         let mid_val = f64::from(arr[mid as usize] as i32);
    //         if mid_val <= target {
    //             begin = mid;
    //         } else {
    //             end = mid;
    //         }
    //     }
    //     let mut i = begin;
    //     while i >= 0 && i >= begin - k {
    //         let sub = (
    //             NonNan(f64::abs(arr[i as usize] as f64 - target)),
    //             arr[i as usize],
    //         );
    //         Solution::cmp_add(&mut heap, sub, k);
    //         i -= 1;
    //     }
    //     i = begin + 1;
    //     while i < (arr.len() as isize) && i <= begin + k {
    //         let sub = (
    //             NonNan(f64::abs(arr[i as usize] as f64 - target)),
    //             arr[i as usize],
    //         );
    //         Solution::cmp_add(&mut heap, sub, k);
    //         i += 1;
    //     }
    //     heap.iter().map(|(_, v)| *v).collect()
    // }

    // pub fn cmp_add(heap: &mut BinaryHeap<(NonNan, i32)>, elem: (NonNan, i32), k: isize) {
    //     if heap.len() < k as usize {
    //         heap.push(elem);
    //     } else {
    //         let top = heap.pop().unwrap();
    //         if top.0 < elem.0 {
    //             heap.push(top);
    //         } else {
    //             heap.push(elem);
    //         }
    //     }
    // }

    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     use NodeOrVal::{Node, Val};
    //     let mut values = vec![];
    //     let mut stack: Vec<NodeOrVal> = vec![Node(root)];
    //     while let Some(node_or_val) = stack.pop() {
    //         match node_or_val {
    //             Node(node) => {
    //                 if let Some(node_ref) = node {
    //                     let node_b = node_ref.borrow();
    //                     stack.push(Node(node_b.right.clone()));
    //                     stack.push(Val(node_b.val));
    //                     stack.push(Node(node_b.left.clone()));
    //                 }
    //             }
    //             Val(val) => {
    //                 values.push(val);
    //             }
    //         }
    //     }
    //     values
    // }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::assert_equivalent;
    use crate::tree;

    #[test]
    fn test_closest_k_values() {
        let tree = tree![4, 2, 5, 1, 3];

        assert_equivalent(&Solution::closest_k_values(tree, 3.714286, 2), &[3, 4]);
    }

    #[test]
    fn test_closest_k_values1() {
        let tree = tree![8, 1];
        assert_equivalent(&Solution::closest_k_values(tree, 6., 1), &[8]);
    }
}
