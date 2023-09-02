/*
 * @lc app=leetcode id=357 lang=rust
 *
 * [357] Count Numbers with Unique Digits
 */

// @lc code=start
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n==0{
        return 1;
        }
        let n = n as usize;
        let mut predp=9;
        let mut ans = 10;
        for i in 1..n{
            predp=predp*(10-i);
            ans+=predp;
        }
        ans as i32
    }
}
// @lc code=end

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n==0{
            return 1
        }
        fn dfs(idx:usize,n:usize,used:&mut Vec<bool>)->i32{
            if idx==n{
                return 0
            }
            let mut ans=0;
            for i in 0..10{
                if i==0 && n>1 && idx==1{
                    continue
                }
                if used[i]{
                    continue
                }
                used[i]=true;
                ans+=dfs(idx+1,n,used)+1;
                used[i]=false;
            }
            ans
        }
        dfs(0,n as usize,&mut vec![false;10])
    }
}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
       let n=n as usize;
       let mut dp=vec![0;n+1];
        for i in 2..=n{
            dp[i]=dp[i-1]*10+(9*10i32.pow(i as u32-2)-dp[i-1])*(i as i32-1)
        }
        10i32.pow(n  as u32)-dp.iter().sum::<i32>()
    }
}