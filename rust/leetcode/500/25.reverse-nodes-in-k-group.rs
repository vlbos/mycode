/*
 * @lc app=leetcode id=25 lang=rust
 *
 * [25] Reverse Nodes in k-Group
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let reversed =
            |head: Option<Box<ListNode>>| -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
                let mut dummy = ListNode::new(0);
                let mut p = head.as_ref();
                for _ in 0..k {
                    if p.is_none() {
                        return (head, None);
                    }
                    p = p.unwrap().next.as_ref();
                }
                let mut p = head;
                for _ in 0..k {
                    if let Some(mut np) = p {
                        p = np.next.take();
                        np.next = dummy.next.take();
                        dummy.next = Some(np);
                    }
                }
                (dummy.next, p)
            };
        let mut p = head;
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        while p.is_some() {
            let (newh, newp) = reversed(p);
            p = newp;
            tail.next = newh;
            while tail.next.as_ref().is_some() {
                tail = tail.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
// @lc code=end
