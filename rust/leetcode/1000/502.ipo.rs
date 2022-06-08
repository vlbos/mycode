/*
 * @lc app=leetcode id=502 lang=rust
 *
 * [502] IPO
 */

// @lc code=start
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut arr: Vec<(i32, i32)> = capital.iter().cloned().zip(profits).collect();
        arr.sort();
        let mut curr = 0;
        let mut q = std::collections::BinaryHeap::new();
        let mut w = w;
        for _ in 0..k {
            while curr < n && arr[curr].0 <= w {
                q.push(arr[curr].1);
                curr += 1;
            }
            if let Some(p) = q.pop() {
                w += p;
            } else {
                break;
            }
        }
        w
    }
}
// @lc code=end
