/*
 * @lc app=leetcode id=224 lang=rust
 *
 * [224] Basic Calculator
 */

// @lc code=start
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut ops = Vec::new();
        ops.push(1);
        let mut sign = 1;
        let mut ans = 0;
        let mut i = 0;
        let bs = s.as_bytes();
        let n = bs.len();
        while i < n {
            match bs[i] {
                b' ' => i += 1,
                b'+' => {
                    sign = ops[ops.len() - 1];
                    i += 1;
                }
                b'-' => {
                    sign = -ops[ops.len() - 1];
                    i += 1;
                }
                b'(' => {
                    ops.push(sign);
                    i += 1;
                }
                b')' => {
                    ops.pop();
                    i += 1;
                }
                _ => {
                    let mut num = 0;
                    while i < n && (bs[i] as char).is_ascii_digit() {
                        num = num * 10 + (bs[i] - b'0') as i64;
                        i += 1;
                    }
                    ans += sign * num;
                }
            }
        }
        ans as _
    }
}
// @lc code=end
