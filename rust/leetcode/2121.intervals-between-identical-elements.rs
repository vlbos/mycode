/*
 * @lc app=leetcode id=2121 lang=rust
 *
 * [2121] Intervals Between Identical Elements
 */

// @lc code=start
impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
       let n = arr.len();
        let mut ans = vec![0i64; n];
        use std::collections::HashMap;
        let mut total = HashMap::new();
        let mut cnt = HashMap::new();
        for i in 0..n {
            let val = arr[i];
            if let Some(&count) = cnt.get(&val) {
                ans[i] += (i as i64) * (count as i64) - *total.get(&val).unwrap_or(&0);
            }
            *total.entry(val).or_insert(0) += i as i64;
            *cnt.entry(val).or_insert(0) += 1;
        }
        total.clear();
        cnt.clear();
        for i in (0..n).rev() {
            let val = arr[i];
            if let Some(&count) = cnt.get(&val) {
                ans[i] += *total.get(&val).unwrap_or(&0) - (i as i64) * (count as i64);
            }
            *total.entry(val).or_insert(0) += i as i64;
            *cnt.entry(val).or_insert(0) += 1;
        }
        ans
    }
}
// @lc code=end
