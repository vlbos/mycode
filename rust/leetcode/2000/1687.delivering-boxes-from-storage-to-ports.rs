/*
 * @lc app=leetcode id=1687 lang=rust
 *
 * [1687] Delivering Boxes from Storage to Ports
 */

// @lc code=start
impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        let n = boxes.len();
        let (mut ws, mut neg) = (vec![0; n + 1], vec![0; n + 1]);
        let mut pp = 0;
        for (i, v) in boxes.iter().enumerate() {
            let (p, w) = (v[0], v[1]);
            if i > 0 {
                neg[i + 1] = neg[i] + if pp != p { 1 } else { 0 };
            }
            ws[i + 1] = ws[i] + w;
            pp = p;
        }
        let (mut f, mut g) = (vec![0; n + 1], vec![0; n + 1]);
        let mut q = std::collections::VecDeque::from([0]);

        for i in 1..=n {
            while !q.is_empty() && i > *q.front().unwrap() + max_boxes as usize
                || ws[i] - ws[*q.front().unwrap()] > max_weight
            {
                q.pop_front();
            }
            f[i] = g[*q.front().unwrap()] + neg[i] + 2;
            if i == n {
                continue;
            }
            g[i] = f[i] - neg[i + 1];
            while !q.is_empty() && g[*q.back().unwrap()] >= g[i] {
                q.pop_back();
            }
            q.push_back(i);
        }
        f[n]
    }
}
// @lc code=end
