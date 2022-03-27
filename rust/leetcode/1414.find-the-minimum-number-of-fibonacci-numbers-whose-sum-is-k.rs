/*
 * @lc app=leetcode id=1414 lang=rust
 *
 * [1414] Find the Minimum Number of Fibonacci Numbers Whose Sum Is K
 */

// @lc code=start
impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut f = vec![1, 1];
        while *f.last().unwrap() < k {
            let n = f.len();
            f.push(f[n - 1] + f[n - 2]);
        }
        let mut i = f.len() - 1;
        let mut ans = 0;
        let mut k = k;
        while k > 0 {
            if k >= f[i] {
                k -= f[i];
                ans += 1;
            }
            i -= 1;
        }
        ans
    }
}
// @lc code=end
