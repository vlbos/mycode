/*
 * @lc app=leetcode id=397 lang=rust
 *
 * [397] Integer Replacement
 */

// @lc code=start
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut ans = 0;
        let mut nn = n as i64;
        while nn > 1 {
            if nn & 1 == 0 {
                nn >>= 1;
            } else if nn&2 == 0||nn==3 {
                nn -= 1;
            } else {
                nn += 1;
            }
            ans += 1;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let s=format!("{:b}",n);
        let bits=s.as_bytes();
        let (mut dp0,mut dp1)=(0,1);
        for i in 1..bits.len(){
            let low=bits[i]-b'0';
            if low==0{
                dp1=dp0.min(dp1)+2;
                dp0+=1;
            }else{
                dp0=dp0.min(dp1)+2;
                dp1+=1;
            }
        }
        dp0
    }
}