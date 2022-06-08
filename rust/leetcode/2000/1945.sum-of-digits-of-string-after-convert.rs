/*
 * @lc app=leetcode id=1945 lang=rust
 *
 * [1945] Sum of Digits of String After Convert
 */

// @lc code=start
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut v = Vec::new();
        for c in s.chars() {
            let i = (c as u8 - 'a' as u8 + 1) as i32;
            if i < 10 {
                v.push(i);
            } else {
                v.push(i / 10);
                v.push(i % 10);
            }
        }
        let sum = v.iter().sum::<i32>();
        if k == 1 {
            return sum;
        }
        let mut sumd = sum;
        for i in 0..k - 1 {
            let mut res = 0;
            while sumd > 0 {
                res += sumd % 10;
                sumd /= 10;
            }
            sumd = res;
        }
        sumd
    }
}
// @lc code=end
