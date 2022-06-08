/*
 * @lc app=leetcode id=1388 lang=rust
 *
 * [1388] Pizza With 3n Slices
 */

// @lc code=start
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let (mut link_l, mut link_r) = (vec![0; n], vec![0; n]);
        for i in 0..n {
            link_l[i] = if i == 0 { n - 1 } else { i - 1 };
            link_r[i] = if i == n - 1 { 0 } else { i + 1 };
        }
        let mut valid = vec![true; n];
        let mut q: std::collections::BinaryHeap<(i32, usize)> =
            slices.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let mut ans = 0;
        let mut slices = slices;
        for _ in 0..n / 3 {
            while !q.is_empty() && !valid[q.peek().unwrap().1] {
                q.pop();
            }
            let pos = q.pop().unwrap().1;
            ans += slices[pos];
            slices[pos] = slices[link_l[pos]] + slices[link_r[pos]] - slices[pos];
            q.push((slices[pos], pos));
            valid[link_l[pos]] = false;
            valid[link_r[pos]] = false;
            link_r[link_l[link_l[pos]]] = pos;
            link_l[link_r[link_r[pos]]] = pos;
            link_l[pos] = link_l[link_l[pos]];
            link_r[pos] = link_r[link_r[pos]];
        }
        ans
    }
}
// @lc code=end
