/*
 * @lc app=leetcode id=166 lang=rust
 *
 * [166] Fraction to Recurring Decimal
 */

// @lc code=start
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let num = numerator as i64;
        let mut den = denominator as i64;
        let s = num / den;
        let mut r = num % den;
        let signednum = if num < 0 { -1 } else { 1 };
        let signeddem = if den < 0 { -1 } else { 1 };
        let mut ans = s.abs().to_string();
        if signednum * signeddem < 0 {
            ans.insert(0, '-');
        }
        if r != 0 {
            ans.push('.');
        }
        let mut m = std::collections::HashMap::<i64, usize>::new();
        r=r.abs();
        den=den.abs();
        while r > 0 {
            if let Some(i) = m.get(&r) {
                ans.insert(*i, '(');
                ans.push(')');
                break;
            } else {
                let t = r * 10;
                m.insert(r, ans.len());
                ans.push((b'0' + (t / den) as u8) as char);
                r = t % den;
                if r == 0 {
                    break;
                }
            }
        }
        ans
    }
}
// @lc code=end
