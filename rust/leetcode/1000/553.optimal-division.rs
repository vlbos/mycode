/*
 * @lc app=leetcode id=553 lang=rust
 *
 * [553] Optimal Division
 */

// @lc code=start
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len()==1{
        return nums[0].to_string();
        }
        if nums.len()==2{
        return format!("{}/{}",nums[0],nums[1]);
        }
        let mut ans = format!("{}/({}",nums[0],nums[1]);
        for n in nums.iter().skip(2){
            ans.push_str(&format!("/{}",n));
        }
        ans.push(')');
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let n=nums.len();
        let mut dp=vec![vec![(10000.0,0.0,String::new(),String::new());n];n];
        for (i,&num) in nums.iter().enumerate(){
            dp[i][i]=(num as f64,num as f64,num.to_string(),num.to_string());
        }
        for i in 0..n{
            for j in 0..n-i{
                for k in j..j+i{
                    if dp[j][j+i].1<dp[j][k].1/dp[k+1][j+i].0{
                        dp[j][j+i].1=dp[j][k].1/dp[k+1][j+i].0;
                        
                        dp[j][j+i].3=format!("{}/{}{}{}",dp[j][k].3,if k+1==j+i{""}else{"("} , dp[k+1][j+i].2,if k+1==j+i{""}else{")"} );
                    }
                    if dp[j][j+i].0>dp[j][k].0/dp[k+1][j+i].1{
                        dp[j][j+i].0=dp[j][k].0/dp[k+1][j+i].1;
                        
                        dp[j][j+i].2=format!("{}/{}{}{}",dp[j][k].2,if k+1==j+i{""}else{"("} , dp[k+1][j+i].3,if k+1==j+i{""}else{")"} );
                    }
                }
            }
        }
        dp[0][n-1].3.clone()
    }
}