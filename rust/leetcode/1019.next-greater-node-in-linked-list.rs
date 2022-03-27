/*
 * @lc app=leetcode id=1019 lang=rust
 *
 * [1019] Next Greater Node In Linked List
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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut p = &head;
        let mut s = Vec::new();
        while let Some(n)=p{
            while !s.is_empty() && ans[*s.last().unwrap()]<n.val{
                ans[*s.last().unwrap()]=n.val;
                s.pop();
            }
            s.push(ans.len());
            ans.push(n.val);
            p=&n.next;
        }
        while let Some(i)=s.pop(){
                ans[i]=0;
        }
        ans
    }
}
// @lc code=end

