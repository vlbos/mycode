/*
 * @lc app=leetcode id=2055 lang=rust
 *
 * [2055] Plates Between Candles
 */

// @lc code=start
impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
       let n = s.len();
        let mut pre_sum = vec![0; n];
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                sum += 1;
            }
            pre_sum[i] = sum;
        }
        let mut left = vec![0; n];
        let mut l = -1;
        for (i, c) in s.chars().enumerate() {
            if c == '|' {
                l = i as i32;
            }
            left[i] = l;
        }
        let mut right = vec![0; n];
        let mut r = -1;
        for (j, c) in s.chars().rev().enumerate() {
            let i =n-j-1;
            if c == '|' {
                r = i as i32 ;
            }
            right[i] = r;
        }
        let mut ans = Vec::new();
        for q in &queries {
            let (x, y) = (right[q[0] as usize], left[q[1] as usize]);
            ans.push(if x == -1 || y == -1 || x >= y {
                0
            } else {
                pre_sum[y as usize] - pre_sum[x as usize]
            });
        }
        ans
    }
}
// @lc code=end
