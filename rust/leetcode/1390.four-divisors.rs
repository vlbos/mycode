/*
 * @lc app=leetcode id=1390 lang=rust
 *
 * [1390] Four Divisors
 */

// @lc code=start
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for &n in &nums {
            let (mut cnt, mut sum) = (0, 0);
            let mut i = 1;
            while i * i <= n {
                if n % i == 0 {
                    cnt += 1;
                    sum += i;
                    if i * i != n {
                        cnt += 1;
                        sum += n / i;
                    }
                }
                i += 1;
            }
            if cnt == 4 {
                ans += sum;
            }
        }
        ans
    }
}
// @lc code=end
