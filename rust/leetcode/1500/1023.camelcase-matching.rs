/*
 * @lc app=leetcode id=1023 lang=rust
 *
 * [1023] Camelcase Matching
 */

// @lc code=start
impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pu = pattern
            .chars()
            .filter(|x| x.is_ascii_uppercase())
            .collect::<String>();
        let ps = pattern
            .split(&pu.chars().collect::<Vec<char>>()[..])
            .collect::<Vec<&str>>();
        let mut ans = Vec::new();
        let can_match = |p: &str, q: &str| -> bool {
            if !p.is_empty() && q.is_empty() {
                return false;
            }
            let (mut i, mut j) = (0, 0);
            while i < p.len() && j < q.len() {
                if p.chars().nth(i).unwrap() == q.chars().nth(j).unwrap() {
                    i += 1;
                    j += 1;
                } else {
                    j += 1;
                }
            }
            i == p.len()
        };
        for q in &queries {
            let qu = q
                .chars()
                .filter(|x| x.is_ascii_uppercase())
                .collect::<String>();
            if qu != pu {
                ans.push(false);
                continue;
            }
            let qs = q
                .split(&qu.chars().collect::<Vec<char>>()[..])
                .collect::<Vec<&str>>();
            let mut flag = true;
            for (i, v) in ps.iter().enumerate() {
                if !can_match(v, qs[i]) {
                    flag = false;
                    break;
                }
            }
            ans.push(flag);
        }
        ans
    }
}
// @lc code=end
