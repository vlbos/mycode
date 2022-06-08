/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);
        let mut p = dummy.clone();
        let mut len = 0;
        while p.next.is_some(){
            p=p.next.unwrap();
            len+=1;
        }
        let mut q = dummy.as_mut();
        for _ in 0..len-n{
             q=q.next.as_mut().unwrap();
        }
        let next = q.next.as_mut().unwrap();
        q.next=next.next.clone();
        dummy.next
    }
}
// @lc code=end

