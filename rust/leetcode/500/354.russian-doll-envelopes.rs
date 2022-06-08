/*
 * @lc app=leetcode id=354 lang=rust
 *
 * [354] Russian Doll Envelopes
 */

// @lc code=start
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by_key(|x| vec![x[0], -x[1]]);
        let mut f = vec![envelopes[0][1]];
        for num in envelopes.iter().skip(1) {
            if num[1] > f[f.len() - 1] {
                f.push(num[1]);
            } else {
                if let Ok(index) | Err(index) = f.binary_search(&num[1]) {
                    f[index] = num[1];
                }
            }
        }
        f.len() as _
    }
}
// @lc code=end
