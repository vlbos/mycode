/*
 * @lc app=leetcode id=2058 lang=rust
 *
 * [2058] Find the Minimum and Maximum Number of Nodes Between Critical Points
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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut p = &head;
        let mut i = 0;
        let mut pre:Vec<i32> = Vec::new();
        let mut min = i32::MAX;
        while let Some(n) = p {
            if pre.len() > 2 {
                pre.remove(0);
            }
            let v = n.val;
            if pre.len() == 2 {
                if pre[1]>v.max(pre[0]) ||pre[1]<v.min(pre[0]) {
                    ans.push(i-1);
                    let l = ans.len();
                    if l > 1 {
                        min = min.min(ans[l - 1] - ans[l - 2]);
                    }
                }
            }
            pre.push(v);
            i += 1;
            p = &n.next;
        }
        if ans.len() < 2 {
            vec![-1, -1]
        } else {
            vec![min, ans[ans.len() - 1] - ans[0]]
        }
    }
}
// @lc code=end
