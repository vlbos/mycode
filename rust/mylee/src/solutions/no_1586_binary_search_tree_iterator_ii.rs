// 1586\. Binary Search Tree Iterator II
// =====================================

// Implement the `BSTIterator` class that represents an iterator over the **[in-order traversal](https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR))** of a binary search tree (BST):

// *   `BSTIterator(TreeNode root)` Initializes an object of the `BSTIterator` class. The `root` of the BST is given as part of the constructor.
// The pointer should be initialized to a non-existent number smaller than any element in the BST.
// *   `boolean hasNext()` Returns `true` if there exists a number in the traversal to the right of the pointer, otherwise returns `false`.
// *   `int next()` Moves the pointer to the right, then returns the number at the pointer.
// *   `boolean hasPrev()` Returns `true` if there exists a number in the traversal to the left of the pointer, otherwise returns `false`.
// *   `int prev()` Moves the pointer to the left, then returns the number at the pointer.

// Notice that by initializing the pointer to a non-existent smallest number, the first call to `next()` will return the smallest element in the BST.

// You may assume that `next()` and `prev()` calls will always be valid. That is, there will be at least a next/previous number in the in-order traversal when `next()`/`prev()` is called.

// **Follow up:** Could you solve the problem without precalculating the values of the tree?

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2020/09/14/untitled-diagram-1.png)**

// **Input**
// \["BSTIterator", "next", "next", "prev", "next", "hasNext", "next", "next", "next", "hasNext", "hasPrev", "prev", "prev"\]
// \[\[\[7, 3, 15, null, null, 9, 20\]\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\], \[null\]\]
// **Output**
// \[null, 3, 7, 3, 7, true, 9, 15, 20, false, true, 15, 9\]

// **Explanation**
// // The underlined element is where the pointer currently is.
// BSTIterator bSTIterator = new BSTIterator(\[7, 3, 15, null, null, 9, 20\]); // state is  \[3, 7, 9, 15, 20\]
// bSTIterator.next(); // state becomes \[3, 7, 9, 15, 20\], return 3
// bSTIterator.next(); // state becomes \[3, 7, 9, 15, 20\], return 7
// bSTIterator.prev(); // state becomes \[3, 7, 9, 15, 20\], return 3
// bSTIterator.next(); // state becomes \[3, 7, 9, 15, 20\], return 7
// bSTIterator.hasNext(); // return true
// bSTIterator.next(); // state becomes \[3, 7, 9, 15, 20\], return 9
// bSTIterator.next(); // state becomes \[3, 7, 9, 15, 20\], return 15
// bSTIterator.next(); // state becomes \[3, 7, 9, 15, 20\], return 20
// bSTIterator.hasNext(); // return false
// bSTIterator.hasPrev(); // return true
// bSTIterator.prev(); // state becomes \[3, 7, 9, 15, 20\], return 15
// bSTIterator.prev(); // state becomes \[3, 7, 9, 15, 20\], return 9

// **Constraints:**

// *   The number of nodes in the tree is in the range `[1, 105]`.
// *   `0 <= Node.val <= 106`
// *   At most 105 calls will be made to `hasNext`, `next`, `hasPrev`, and `prev`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)
/*
 * @lc app=leetcode id=173 lang=rust
 *
 * [173] Binary Search Tree Iterator
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
use super::util::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
struct BSTIterator {
    s: Vec<Rc<RefCell<TreeNode>>>,
    p: Vec<Rc<RefCell<TreeNode>>>,
    index: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut s = Vec::new();
        let mut r = root;
        while let Some(n) = r {
            r = n.borrow().left.clone();
            s.push(n);
        }
        Self {
            s,
            p: Vec::new(),
            index: -1,
        }
    }

    fn prev(&mut self) -> i32 {
        self.index -= 1;
        self.p[self.index as usize].borrow().val
    }
    fn next(&mut self) -> i32 {
        let val;
        let i = (self.index + 1) as usize;
        if i >= 0 && i < self.p.len() {
            val = self.p[i].borrow().val;
        } else {
            let node = self.s.pop().unwrap();
            self.p.push(node.clone());
            val = node.borrow().val;
            let mut r = node.borrow().right.clone();
            while let Some(n) = r {
                r = n.borrow().left.clone();
                self.s.push(n);
            }
        }
        self.index += 1;
        val
    }
    fn has_prev(&self) -> bool {
        self.index > 0 && self.index < self.p.len() as i32
    }
    fn has_next(&self) -> bool {
        let i = (self.index + 1) as usize;
        if i < self.p.len() {
            return true;
        }

        !self.s.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::tree;
    #[test]
    pub fn test_bstiterator_1() {
        let mut it = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
        assert_eq!(3, it.next());
        assert_eq!(7, it.next());
        assert_eq!(3, it.prev());
        assert_eq!(7, it.next());
        assert!(it.has_next());
        assert_eq!(9, it.next());
        assert_eq!(15, it.next());
        assert_eq!(20, it.next());
        assert!(!it.has_next());
        assert!(it.has_prev());
        assert_eq!(15, it.prev());
        assert_eq!(9, it.prev());
    }
}
