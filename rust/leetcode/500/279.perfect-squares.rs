/*
 * @lc app=leetcode id=279 lang=rust
 *
 * [279] Perfect Squares
 */

// @lc code=start
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp =vec![0;n+1];
        for i in 1..=n{
            let mut min =i32::MAX;
            let mut j = 1;
            while j*j<=i{
                min = min.min(dp[i-j*j]);
                j+=1;
            }
            dp[i]=min+1;
        }
        dp[n]
    }
}
// @lc code=end

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // let n= n as usize;
        // let mut f=vec![0;n+1];
        // for i in 1..=n{
        //     let mut min=i32::MAX;
        //     let mut j=1;
        //     while j*j<=i{
        //         min=min.min(f[i-j*j]+1);
        //         j+=1;
        //     }
        //     f[i]=min;
        // }
        // f[n]
        let is_perfect_answer=|x:i32|{
                let y=(x as f64).sqrt() as i32;
                y*y==x
        };
        let check_answer4=|mut x:i32|{
               while x%4==0{
                   x/=4;
               }
               x%8==7
        };
        if is_perfect_answer(n){
            return 1
        }
        if check_answer4(n){
            return 4
        }
        let mut i=1;
        while i*i<n{
            if is_perfect_answer(n-i*i){
                return 2
            }
            i+=1;
        }
        3
    }
}