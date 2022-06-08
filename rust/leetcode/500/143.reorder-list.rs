/*
 * @lc app=leetcode id=143 lang=rust
 *
 * [143] Reorder List
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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none()|| head.as_ref().unwrap().next.as_ref().unwrap().next.is_none(){
            return;
        }
        let mut v = Vec::new();
        let mut p = head.as_deref_mut().unwrap().next.take();
        while let Some(mut n) = p.take() {
            p = n.next.take();
            v.push(Some(n));
        }
        let mut i = 0;
        let mut j = v.len() - 1;
        let mut p = &mut head.as_deref_mut().unwrap().next;
        while i < j {
            *p = v[j].take();
            p = &mut p.as_deref_mut().unwrap().next;
            j -= 1;
            if i == j {
                break;
            }
            *p = v[i].take();
            p = &mut p.as_deref_mut().unwrap().next;
            i += 1;
        }
        *p = v[i].take();
    }
}
// @lc code=end
