/*
 * @lc app=leetcode id=1090 lang=rust
 *
 * [1090] Largest Values From Labels
 */

// @lc code=start
impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut vl = values
            .iter()
            .zip(&labels)
            .map(|x| (*x.0, *x.1))
            .collect::<Vec<(i32, i32)>>();
        vl.sort_by(|a, b| b.0.cmp(&a.0));
        let mut m = std::collections::HashMap::<i32, i32>::new();
        let mut ans = 0;
        let mut cnt = 0;
        for (v, l) in &vl {
            if cnt == num_wanted {
                break;
            }
            if let Some(lc) = m.get_mut(&l) {
                if *lc < use_limit {
                    ans += v;
                    cnt += 1;
                    *lc += 1;
                }
            } else {
                ans += v;
                cnt += 1;
                m.insert(*l, 1);
            }
        }
        ans
    }
}
// @lc code=end
