/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn inner_reverse_list(
            reversed: Option<Box<ListNode>>,
            head: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut _head) = head {
                let new_head = std::mem::replace(&mut _head.next, reversed);
                inner_reverse_list(Some(_head), new_head)
            } else {
                reversed
            }
        }
        inner_reverse_list(None, head)
    }
}
// @lc code=end

