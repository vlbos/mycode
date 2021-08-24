/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
            if head.is_none() {
            return None;
            }
            let mut head = head;
            let mut _head =  head.as_mut().unwrap();
            _head.next = Self::remove_elements(_head.next.take(),val);
            if _head.val == val{
            return    _head.next.clone() ;
            }
            head

        //     let mut ddb = Box::new(ListNode::new(0));
        //     ddb.next = head;
        //     let mut h = Some(ddb);
        //     let mut p1 = h.as_mut().unwrap();
        //     while let Some(p2) = p1.next.as_mut() {
        //         if val == p2.val {
        //             p1.next = p2.next.take();
        //         } else {
        //             p1 = p1.next.as_mut().unwrap();
        //         }
        //     }

        //   h.unwrap().next
    }
}
// @lc code=end

