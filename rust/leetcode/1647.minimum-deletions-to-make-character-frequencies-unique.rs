/*
 * @lc app=leetcode id=1647 lang=rust
 *
 * [1647] Minimum Deletions to Make Character Frequencies Unique
 */

// @lc code=start
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut a = std::collections::HashMap::new();
        for b in s.bytes() {
            *a.entry(b - b'a').or_insert(0) += 1;
        }
        let mut p = std::collections::HashSet::new();
        let mut aa = a.iter().map(|x| (*x.1, *x.0)).collect::<Vec<(i32, u8)>>();
        aa.sort();
        let mut ans = 0;
        for &(f, b) in &aa {
            if p.contains(&f) {
                let mut i = f - 1;
                while i > 0 {
                    if !p.contains(&i) {
                        break;
                    }
                    i -= 1;
                }
                p.insert(i);
                ans += f - i;
            } else {
                p.insert(f);
            }
        }
        ans
    }
}
// @lc code=end
