/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let r = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        let d = vec![1, 5, 10, 50, 100, 500, 1000];
        let mut ans = String::new();
        let mut i = d.len() - 1;
        let mut n = num;
        while i > 1 {
            if n >= d[i] {
                ans.push_str(&r[i].to_string().repeat((n / d[i]) as usize));
                n %= d[i];
            } else if n >= d[i] - d[i - 2] {
                ans.push(r[i - 2]);
                ans.push(r[i]);
                n -= d[i] - d[i - 2];
            } else if n >= d[i - 1] {
                ans.push(r[i - 1]);
                ans.push_str(
                    &r[i - 2]
                        .to_string()
                        .repeat(((n - d[i - 1]) / d[i - 2]) as usize),
                );
                n -= n / d[i - 2] * d[i - 2];
            } else if n >= d[i - 1] - d[i - 2] {
                ans.push(r[i - 2]);
                ans.push(r[i - 1]);
                n -= d[i - 1] - d[i - 2];
            } else {
                i -= 2;
            }
        }
        if n > 0 {
            ans.push_str(&r[0].to_string().repeat(n as usize));
        }
        ans
    }
}
// @lc code=end
