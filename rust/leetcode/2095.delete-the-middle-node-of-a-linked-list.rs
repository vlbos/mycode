/*
 * @lc app=leetcode id=2095 lang=rust
 *
 * [2095] Delete the Middle Node of a Linked List
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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
         if head.as_ref().unwrap().next.is_none() {
            return None;
        }
        let mut p = &head;
        let mut len = 0;
        while let Some(node)=p {
            p = &node.next;
            len+=1;
        }
        len/=2;
        let mut head=head;
        let mut p =&mut head;
        for _ in 0..len{
            if let Some(node)=p{
                p=&mut node.next;
            }
        }
        *p=p.take().unwrap().next;
        head
    }
}
// @lc code=end
