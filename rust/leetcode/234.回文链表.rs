/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::iter;
use std::mem;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let half = length / 2;
        let mut reversed = None;

        for _ in 0..half {
            let mut node = head.unwrap();

            head = mem::replace(&mut node.next, reversed);
            reversed = Some(node);
        }

        if length % 2 != 0 {
            head = head.unwrap().next;
        }

        reversed == head
    }
}
// @lc code=end

