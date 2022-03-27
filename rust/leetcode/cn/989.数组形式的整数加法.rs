/*
 * @lc app=leetcode.cn id=989 lang=rust
 *
 * [989] 数组形式的整数加法
 */

// @lc code=start
impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num = num;
        let mut k = k;
        let mut inc = 0;
        let mut i = num.len() - 1;
        while k > 0 || i >= 0 {
            let d = k % 10;
            let mut v = num[i] + d + inc;
            num[i] = v % 10;
            inc = v / 10;
            k /= 10;
            if i == 0 {
                if k > 0 {
                    num.insert(0, 0);
                } else {
                    break;
                }
            } else {
                if i > 0 {
                    i -= 1;
                } else {
                    break;
                }
            }
        }
        if inc > 0 {
            num.insert(0, inc);
        }

        num
    }
}
// @lc code=end
