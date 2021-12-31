/*
 * @lc app=leetcode id=86 lang=rust
 *
 * [86] Partition List
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
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut p = head;
        let mut l = None;
        let mut r = None;
        let mut left_tail = &mut l;
        let mut right_tail = &mut r;

        while let Some(mut n) = p {
            p = n.next.take();

            if n.val < x {
                left_tail = &mut left_tail.get_or_insert(n).next;
            } else {
                right_tail = &mut right_tail.get_or_insert(n).next;
            }
        }

        *left_tail = r;

        l
    }
}
// @lc code=end
