/*
 * @lc app=leetcode id=1851 lang=rust
 *
 * [1851] Minimum Interval to Include Each Query
 */

// @lc code=start
impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut events = Vec::new();
        for (i, &query) in queries.iter().enumerate() {
            events.push(vec![1, query, i as i32]);
        }
        for (i, v) in intervals.iter().enumerate() {
            events.push(vec![0, v[0], v[1]]);
            events.push(vec![2, v[1], v[0]]);
        }
        events.sort_by(|a, b| {
            if a[1] == b[1] {
                a[0].cmp(&b[0])
            } else {
                a[1].cmp(&b[1])
            }
        });
        let mut ans = vec![-1; queries.len()];
        let mut seg = std::collections::BTreeMap::new();
        for event in &events {
            if event[0] == 0 {
                *seg.entry(event[2] - event[1] + 1).or_insert(0) += 1;
            } else if event[0] == 1 {
                if let Some(&k) = seg.keys().next() {
                    ans[event[2] as usize] = k;
                }
            } else {
                if let Some(v) = seg.remove(&(event[1] - event[2] + 1)) {
                    if v > 1 {
                        seg.insert(event[1] - event[2] + 1, v - 1);
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
