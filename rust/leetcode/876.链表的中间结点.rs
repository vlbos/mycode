/*
 * @lc app=leetcode.cn id=876 lang=rust
 *
 * [876] 链表的中间结点
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &head;
        let mut q = &head;
        while p.is_some() && p.as_ref().unwrap().next.is_some(){
             q=&q.as_ref().unwrap().next;
             p = &p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        q.clone()
    }
}
// @lc code=end

