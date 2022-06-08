/*
 * @lc app=leetcode id=811 lang=rust
 *
 * [811] Subdomain Visit Count
 */

// @lc code=start
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = std::collections::HashMap::<String, i32>::new();
        for cp in cpdomains {
            let vs: Vec<String> = cp
                .split_ascii_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let mut ds = Vec::<String>::new();
            let mut ss = vs[1].clone();
            ds.push(ss.clone());
            while let Some(d) = ss.find(".") {
                ss = (&ss[d + 1..]).to_string();
                ds.push(ss.clone());
            }

            for d in &ds {
                let i = vs[0].parse::<i32>().unwrap();
                if let Some(_m) = m.get_mut(d) {
                    *_m += i;
                } else {
                    m.insert((*d).clone(), i);
                }
            }
        }
        let mut r = Vec::<String>::new();
        for (k, v) in m {
            r.push(v.to_string() + " " + &k);
        }
        r
    }
}
// @lc code=end

