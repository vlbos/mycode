/*
 * @lc app=leetcode id=949 lang=rust
 *
 * [949] Largest Time for Given Digits
 */

// @lc code=start
impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut ans = -1;
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                for k in 0..4 {
                    if i == k || k == j {
                        continue;
                    }
                    let l = 6 - i - j - k;
                    let hour = arr[i] * 10 + arr[j];
                    let mintue = arr[k] * 10 + arr[l];
                    if hour < 24 && mintue < 60 {
                        ans = ans.max(hour * 60 + mintue);
                    }
                }
            }
        }
        if ans >= 0 {
            format!("{:02}:{:02}", ans / 60, ans % 60)
        } else {
            String::new()
        }
    }
}
// @lc code=end
