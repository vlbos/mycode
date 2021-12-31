/*
 * @lc app=leetcode id=61 lang=rust
 *
 * [61] Rotate List
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 || head.is_none() {
            return head;
        }
        let mut len = 0;
        let mut p = &head;
        while let Some(_p) = p {
            p = &_p.next;
            len += 1;
        }
        let mut k = k % len;
        if k == 0 {
            return head;
        }
        let mut head = head;
        let mut p = &mut head;

        for _ in 0..len - k {
            if let Some(ref mut _p) = p {
                p = &mut _p.next;
            }
        }
        let mut p = p.take();

        let mut q = &mut p;
        while let Some(_q) = q {
            q = &mut _q.next;
        }

        let qq = q;
        *qq = head;
        p
    }
}
// @lc code=end
