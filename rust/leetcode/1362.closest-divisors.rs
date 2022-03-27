/*
 * @lc app=leetcode id=1362 lang=rust
 *
 * [1362] Closest Divisors
 */

// @lc code=start
impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let divide=|x:i32|->i32{
            let xx = (x as f64).sqrt() as i32;
            for i in (0..=xx).rev(){
                if x%i==0{
                return i;
                }
            }
            0
        };
        let mut ans =vec![0i32,1000_000_000i32];
        for &i in &[num+1,num+2]{
             let t:i32 = divide(i);
             let ti:i32 = i/t;
             if (t-ti).abs()<(ans[0] -ans[1]).abs(){
                ans=vec![t,ti];
             }
        }
        ans
    }
}
// @lc code=end

