/*
 * @lc app=leetcode id=2081 lang=rust
 *
 * [2081] Sum of k-Mirror Numbers
 */

// @lc code=start
impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
       let is_palindrome = |mut x: i64| {
            let mut digit = Vec::new();
            let k =  k as i64;
            while x > 0 {
                digit.push(x %k);
                x /= k;
            }
            let mut d = digit.clone();
            d.reverse();
            d == digit
        };
        let (mut left, mut count) = (1, 0);
        let mut ans = 0;
        while count < n {
            let right = left * 10;
            for op in [0, 1] {
                for i in left..right {
                    if count == n {
                        break;
                    }
                    let mut combined = i as i64;
                    let mut x = if op == 0 { combined / 10 } else { combined };
                    while x > 0 {
                        combined = combined * 10 + x % 10;
                        x /= 10;
                    }
                    if is_palindrome(combined) {
                        count += 1;
                        ans += combined;
                    }
                }
            }
            left = right;
        }
        ans
    }
}
// @lc code=end
