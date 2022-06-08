/*
 * @lc app=leetcode id=1882 lang=rust
 *
 * [1882] Process Tasks Using Servers
 */

// @lc code=start
impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut idle = servers
            .iter()
            .enumerate()
            .map(|(i, &v)| Reverse((v, i)))
            .collect::<BinaryHeap<Reverse<(i32, usize)>>>();
        let mut busy = BinaryHeap::new();
        let mut ts = 0i64;
        let release = |busy: &mut BinaryHeap<Reverse<(i64, usize)>>,
                       idle: &mut BinaryHeap<Reverse<(i32, usize)>>,ts:&i64| {
            while !busy.is_empty() && busy.peek().unwrap().0.0  <= *ts {
                if let Some(Reverse((_, idx))) = busy.pop() {
                    idle.push(Reverse((servers[idx], idx)));
                }
            }
        };
        let mut ans = Vec::new();
        for i in 0..tasks.len() {
            ts = ts.max(i as i64);
            release(&mut busy, &mut idle,&ts);
            if idle.is_empty() {
                ts = busy.peek().unwrap().0.0;
                release(&mut busy, &mut idle,&ts);
            }
            if let Some(Reverse((_, idx))) = idle.pop() {
                ans.push(idx as i32);
                busy.push(Reverse((ts + tasks[i] as i64, idx)));
            }
        }
        ans
    }
}
// @lc code=end
