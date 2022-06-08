/*
 * @lc app=leetcode id=1781 lang=rust
 *
 * [1781] Sum of Beauty of All Substrings
 */

// @lc code=start
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let n = s.len();
        if n < 3 {
            return 0;
        }
        let mut ans = 0;
        for i in 0..n {
            let mut cnt = vec![0; 26];
            let bs = s.as_bytes();
            for j in i..n {
                cnt[(bs[j] - b'a') as usize] += 1;
                let min = cnt.iter().filter(|x| (*x) > &0).min().unwrap();
                let max = cnt.iter().filter(|x| (*x) > &0).max().unwrap();
                ans += max - min;
            }
        }
        ans
    }
}
// @lc code=end
