/*
 * @lc app=leetcode id=997 lang=rust
 *
 * [997] Find the Town Judge
 */

// @lc code=start
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.is_empty() {
            return 1;
        }
        let mut j = Vec::new();
        let mut p = Vec::new();
        for t in &trust {
            if !j.contains(&t[1]) {
                j.push(t[1]);
            }
            if !p.contains(&t[0]) {
                p.push(t[0]);
            }
        }
        for i in 0..j.len() {
            if p.contains(&j[i]) {
                continue;
            }
            let mut c = 0;
            for t in &trust {
                if t[1] == j[i] {
                    c += 1;
                }
                if c == n - 1 {
                    return j[i];
                }
            }
        }
        -1
    }
}
// @lc code=end
