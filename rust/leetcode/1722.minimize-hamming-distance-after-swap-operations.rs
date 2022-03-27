/*
 * @lc app=leetcode id=1722 lang=rust
 *
 * [1722] Minimize Hamming Distance After Swap Operations
 */

// @lc code=start
impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        use std::collections::HashMap;
        let mut g: HashMap<usize, Vec<usize>> = HashMap::new();
        for a in allowed_swaps {
            let (i, j) = (a[0] as usize, a[1] as usize);
            g.entry(i).or_default().push(j);
            g.entry(j).or_default().push(i);
        }
        let mut cnt = 0;
        let mut group = vec![0; n];
        for i in 0..n {
            if group[i] > 0 {
                continue;
            }
            cnt += 1;
            group[i] = cnt;
            let mut s = vec![i];
            while let Some(cur) = s.pop() {
                for &j in g.get(&cur).unwrap_or(&Vec::<usize>::new()) {
                    if group[j] == 0 {
                        group[j] = cnt;
                        s.push(j);
                    }
                }
            }
        }
        let mut s = vec![Vec::new(); cnt + 1];
        let mut t = vec![Vec::new(); cnt + 1];
        for i in 0..n {
            s[group[i]].push(source[i]);
            t[group[i]].push(target[i]);
        }
        let mut ans = 0;
        for i in 0..cnt + 1 {
            let mut sm = HashMap::new();
            for &v in &s[i] {
                *sm.entry(v).or_insert(0) += 1;
            }
            let mut tm = HashMap::new();
            for &v in &t[i] {
                *tm.entry(v).or_insert(0) += 1;
            }
            for (k, &v) in &sm {
                ans += (v - *tm.get(k).unwrap_or(&0)).max(0);
            }
        }
        ans
    }
}
// @lc code=end
