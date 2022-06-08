/*
 * @lc app=leetcode id=282 lang=rust
 *
 * [282] Expression Add Operators
 */

// @lc code=start
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
      fn back_track(
            num: &[u8],
            target: i64,
            expr: &mut Vec<u8>,
            i: usize,
            res: i64,
            mul: i64,
            ans: &mut Vec<String>,
        ) {
            if i == num.len() {
                if res == target {
                    ans.push(String::from_utf8(expr.to_vec()).unwrap());
                    return;
                }
            }
            let sign_index = expr.len();
            if i > 0 {
                expr.push(b' ');
            }
            let mut val = 0;
            for j in i..num.len() {
                if j > i && num[i] == b'0' {
                    break;
                }
                val = val * 10 + (num[j] - b'0') as i64;
                expr.push(num[j]);
                if i == 0 {
                    back_track(num, target, expr, j + 1, val, val, ans);
                } else {
                    expr[sign_index] = b'+';
                    back_track(num, target, expr, j + 1, res + val, val, ans);
                    expr[sign_index] = b'-';
                    back_track(num, target, expr, j + 1, res - val, -val, ans);
                    expr[sign_index] = b'*';
                    back_track(num, target, expr, j + 1, res - mul + mul * val,mul * val, ans);
                }
            }
            expr.split_off(sign_index);
        }
        let mut expr = Vec::new();
        let mut ans = Vec::new();
        back_track(
            num.as_bytes(),
            target as i64,
            &mut expr,
            0,
            0,
            0,
            &mut ans,
        );
        ans
    }
}
// @lc code=end
