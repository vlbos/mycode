/*
 * @lc app=leetcode id=935 lang=rust
 *
 * [935] Knight Dialer
 */

// @lc code=start
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let moves=vec![vec![4,6],vec![6,8],vec![7,9],vec![4,8],vec![0,3,9],vec![],vec![0,1,7],vec![2,6],vec![1,3],vec![2,4]];
        let mut dp =[1;10];
        let modmax = 1_000_000_007;
        for _ in 0..n-1{
                let mut dp2 =[0;10];
                for (i,v) in dp.iter().enumerate(){
                    for &x in &moves[i]{
                        let x = x as usize;
                        dp2[x]+=*v as i64;
                        dp2[x]%=modmax;
                    }
                }
                dp=dp2;
        }
        (dp.iter().sum::<i64>()%modmax ) as _
    }
}
// @lc code=end

