// 1756\. Design Most Recently Used Queue
// ======================================

// Design a queue-like data structure that moves the most recently used element to the end of the queue.

// Implement the `MRUQueue` class:

// *   `MRUQueue(int n)` constructs the `MRUQueue` with `n` elements: `[1,2,3,...,n]`.
// *   `fetch(int k)` moves the `kth` element **(1-indexed)** to the end of the queue and returns it.

// **Example 1:**

// **Input:**
// \["MRUQueue", "fetch", "fetch", "fetch", "fetch"\]
// \[\[8\], \[3\], \[5\], \[2\], \[8\]\]
// **Output:**
// \[null, 3, 6, 2, 2\]

// **Explanation:**
// MRUQueue mRUQueue = new MRUQueue(8); // Initializes the queue to \[1,2,3,4,5,6,7,8\].
// mRUQueue.fetch(3); // Moves the 3rd element (3) to the end of the queue to become \[1,2,4,5,6,7,8,3\] and returns it.
// mRUQueue.fetch(5); // Moves the 5th element (6) to the end of the queue to become \[1,2,4,5,7,8,3,6\] and returns it.
// mRUQueue.fetch(2); // Moves the 2nd element (2) to the end of the queue to become \[1,4,5,7,8,3,6,2\] and returns it.
// mRUQueue.fetch(8); // The 8th element (2) is already at the end of the queue so just return it.

// **Constraints:**

// *   `1 <= n <= 2000`
// *   `1 <= k <= n`
// *   At most `2000` calls will be made to `fetch`.

// **Follow up:** Finding an `O(n)` algorithm per `fetch` is a bit easy. Can you find an algorithm with a better complexity for each `fetch` call?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//   public MRUQueue(int n) {

//     public int fetch(int k) {

use super::util::tree::TreeNode;

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_equivalence(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        use std::collections::HashMap;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, freq: &mut HashMap<i32, i32>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            if (node.val as u8 as char).is_ascii_alphabetic() {
                *freq.entry(node.val).or_insert(0) += v;
            } else {
                dfs(&node.left, v, freq);
                dfs(&node.right, v, freq);
            }
        }
        let mut freq = HashMap::new();
        dfs(&root1, 1, &mut freq);
        dfs(&root2, -1, &mut freq);
        if freq.values().any(|v| *v > 0) {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::tree;
    use super::super::util::tree::to_tree;
    fn to_exp_tree(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        to_tree(
            s.split(',')
                .map(|x| {
                    if x == "null" {
                        None
                    } else {
                        Some(x.as_bytes()[0] as i32)
                    }
                })
                .collect::<Vec<Option<i32>>>(),
        )
    }
    #[test]
    pub fn test_check_equivalence_1() {
        assert!(Solution::check_equivalence(
            to_exp_tree("x"),
            to_exp_tree("x")
        ));
    }
    #[test]
    pub fn test_check_equivalence_2() {
        assert!(Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,c,a")
        ));
    }
    #[test]
    pub fn test_check_equivalence_3() {
        assert!(!Solution::check_equivalence(
            to_exp_tree("+,a,+,null,null,b,c"),
            to_exp_tree("+,+,b,d,a")
        ));
    }
}
