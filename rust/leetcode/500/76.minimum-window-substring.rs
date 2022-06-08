/*
 * @lc app=leetcode id=76 lang=rust
 *
 * [76] Minimum Window Substring
 */

// @lc code=start
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        let (mut tm, mut sm) = (HashMap::new(), HashMap::new());
        for c in t.bytes() {
            *tm.entry(c).or_insert(0) += 1;
        }
        let mut l = 0;
        let n = s.len();
        let mut len = n + 1;
        let mut ans = vec![n, n];
        let bs = s.as_bytes();
        let check = |sm:&HashMap<u8,i32>| {
            for (k, &v) in &tm {
                if *sm.get(k).unwrap_or(&0) < v {
                    return false;
                }
            }
            true
        };
        for r in 0..n {
            if tm.contains_key(&bs[r]) {
                *sm.entry(bs[r]).or_insert(0) += 1;
            }
            while  l <= r {
                if !check(&sm){
                    break;
                }
                if r - l + 1 < len {
                    len = r - l + 1;
                    ans = vec![l, r + 1];
                }
                if tm.contains_key(&bs[l]) {
                    if *sm.get(&bs[l]).unwrap_or(&0) < 2 {
                        sm.remove(&bs[l]);
                    } else {
                        *sm.entry(bs[l]).or_insert(0) -= 1;
                    }
                }
                l += 1;
            }
        }
        if ans[0] == n {
            String::new()
        } else {
            s[ans[0]..ans[1]].to_string()
        }
    }
}
// @lc code=end
