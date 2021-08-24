/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len()-1;
         while i> 0 && (digits[i]+1)/10>0 {
            digits[i] = (digits[i]+1)%10;
            i-=1;
        }
        if 0==i && (digits[i]+1)/10>0{
            digits.push(0);
            digits[i] = 0;
        }

        digits[i] += 1;

        digits

    }
}
// @lc code=end

