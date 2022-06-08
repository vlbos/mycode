/*
 * @lc app=leetcode id=400 lang=rust
 *
 * [400] Nth Digit
 */

// @lc code=start
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut digits = 1i64 ;
        let mut digitscount=9i64;
        let mut n = n as i64;
        while  n-digits*digitscount>0{
            n-=digits*digitscount;
            digits+=1;
            digitscount*=10;
        }
        let num = 10i64.pow(digits as u32-1)+(n-1)/digits;
        let mut pos = n%digits;
        pos =  if pos==0 {digits}else{pos};
        (num/10i64.pow((digits-pos) as u32)%10)  as i32
    }
}
// @lc code=end

