/*
 * @lc app=leetcode id=838 lang=rust
 *
 * [838] Push Dominoes
 */

// @lc code=start
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let mut forces = vec![0; n];
        let mut force = 0;
        for (i, v) in dominoes.chars().enumerate() {
            if v == 'R' {
                force = n as i32;
            } else if v == 'L' {
                force = 0;
            } else {
                force = (force - 1).max(0);
            }
            forces[i] += force;
        }

        force = 0;
        for (i, v) in dominoes.chars().rev().enumerate() {
            if v == 'L' {
                force = n as i32;
            } else if v == 'R' {
                force = 0;
            } else {
                force = (force - 1).max(0);
            }
            forces[n - i - 1] -= force;
        }
        let mut ans = String::new();
        for f in &forces {
            ans.push(if *f > 0 {
                'R'
            } else if *f < 0 {
                'L'
            } else {
                '.'
            });
        }
        ans
    }
}
// @lc code=end
