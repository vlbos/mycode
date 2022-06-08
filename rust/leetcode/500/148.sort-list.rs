/*
 * @lc app=leetcode id=148 lang=rust
 *
 * [148] Sort List
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn split_at(
            mut head: Option<Box<ListNode>>,
            n: usize,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut cursor = &mut head;

            for _ in 0..n {
                cursor = &mut cursor.as_deref_mut().unwrap().next;
            }

            let rest = cursor.take();

            (head, rest)
        }

        fn merge(
            mut left: Box<ListNode>,
            mut right: Box<ListNode>,
            mut target: &mut Option<Box<ListNode>>,
        ) -> &mut Option<Box<ListNode>> {
            loop {
                if right.val < left.val {
                    target = &mut target.get_or_insert(right).next;

                    if let Some(next_right) = target.take() {
                        right = next_right;
                    } else {
                        target = &mut target.get_or_insert(left).next;

                        break;
                    }
                } else {
                    target = &mut target.get_or_insert(left).next;

                    if let Some(next_left) = target.take() {
                        left = next_left;
                    } else {
                        target = &mut target.get_or_insert(right).next;

                        break;
                    }
                }
            }

            while let Some(node) = target {
                target = &mut node.next;
            }

            target
        }
        let mut head = head;
        let length = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let mut group_length = 1;

        while group_length < length {
            let next_group_length = group_length * 2;
            let mut next_head = None;
            let mut tail = &mut next_head;

            for _ in 0..length / next_group_length {
                let (group_1, rest) = split_at(head, group_length);
                let (group_2, rest) = split_at(rest, group_length);

                tail = merge(group_1.unwrap(), group_2.unwrap(), tail);
                head = rest;
            }

            if length % next_group_length <= group_length {
                *tail = head;
            } else {
                let (group_1, group_2) = split_at(head, group_length);

                merge(group_1.unwrap(), group_2.unwrap(), tail);
            }

            head = next_head;
            group_length = next_group_length;
        }

        head
    }
}
// @lc code=end
