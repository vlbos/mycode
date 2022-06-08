/*
 * @lc app=leetcode id=2071 lang=rust
 *
 * [2071] Maximum Number of Tasks You Can Assign
 */

// @lc code=start
impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        let (n, m) = (tasks.len(), workers.len());
        let (mut tasks, mut workers) = (tasks, workers);
        tasks.sort();
        workers.sort();
        let check = |mid: usize| {
            let mut p = pills;
            let mut ws = std::collections::VecDeque::new();
            let mut j = m - 1;
            for &task in tasks[..mid].iter().rev() {
                while j + mid >= m && workers[j] + strength >= task {
                    ws.push_front(workers[j]);
                    j -= 1;
                }
                if ws.is_empty() {
                    return false;
                }
                if *ws.back().unwrap() >= task {
                    ws.pop_back();
                } else {
                    if p == 0 {
                        return false;
                    }
                    p -= 1;
                    ws.pop_front();
                }
            }
            true
        };
        let mut ans = 0;
        let (mut left, mut right) = (1, m.min(n));
        while left <=right {
            let mid = (left + right) / 2;
            if check(mid) {
                ans = mid as i32;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
