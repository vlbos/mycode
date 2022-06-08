/*
 * @lc app=leetcode id=1520 lang=rust
 *
 * [1520] Maximum Number of Non-Overlapping Substrings
 */

// @lc code=start
impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let mut seg = vec![(-1, -1); 26];
        for (i, b) in s.bytes().enumerate() {
            let idx = (b - b'a') as usize;
            let ii = i as i32;
            if seg[idx].0 == -1 {
                seg[idx] = (ii, ii);
            } else {
                seg[idx].1 = ii;
            }
        }

        let bs = s.as_bytes();
        for i in 0..26 {
            if seg[i].0 == -1 {
                continue;
            }
            let mut j = seg[i].0;
            while j <= seg[i].1 {
                let idx = (bs[j as usize] - b'a') as usize;
                if seg[i].0 <= seg[idx].0 && seg[idx].1 <= seg[i].1 {
                    j+=1;
                    continue;
                }
                seg[i].0 = seg[i].0.min(seg[idx].0);
                seg[i].1 = seg[i].1.max(seg[idx].1);
                j = seg[i].0 + 1;
            }
        }
        seg.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        let mut ans = Vec::new();
        let mut end = -1;
        for (left, right) in seg {
            if left == -1 {
                continue;
            }
            if end == -1 || left > end {
                end = right;
                ans.push(s[left as usize..=right as usize].to_string());
            }
        }
        ans
    }
}
// @lc code=end
