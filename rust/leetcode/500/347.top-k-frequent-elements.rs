/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for n in &nums {
            *m.entry(n).or_insert(0) += 1;
        }
        fn min_heapy(h: &mut Vec<(i32, i32)>, i: usize) {
            let l = (i + 1) * 2 - 1;
            let r = (i + 1) * 2;
            let mut smallest = i;
            if l < h.len() && h[l].1 < h[smallest].1 {
                smallest = l;
            }
            if r < h.len() && h[r].1 < h[smallest].1 {
                smallest = r;
            }
            if smallest != i {
                let t = h[smallest];
                h[smallest] = h[i];
                h[i] = t;
            }
        }

        let mut h = Vec::<(i32, i32)>::new();
        for (n, v) in m {
            if h.len() < k as usize {
                h.push((*n, v));
                for i in (0..=h.len() / 2).rev() {
                    min_heapy(&mut h, i);
                }
            } else if v > h[0].1 {
                h[0] = (*n, v);
                for i in (0..=h.len() / 2).rev() {
                    min_heapy(&mut h, i);
                }
            }
        }
        h.iter().map(|x| x.0).collect()
    }
}
// @lc code=end
