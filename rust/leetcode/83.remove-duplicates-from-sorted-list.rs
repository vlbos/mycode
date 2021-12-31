/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
         let mut h = head;
        let mut p = &mut h;
        while let Some(ref mut _p) = p {
            let mut q = _p.next.take();
            while let Some(mut _q) = q {
                if &_p.val == &_q.val {
                    q = _q.next.take();
                } else {
                    _p.next = Some(_q);
                    break;
                }
            }
            p = &mut _p.next;
        }
        h
    }
}
// @lc code=end

