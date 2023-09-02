/*
 * @lc app=leetcode id=907 lang=rust
 *
 * [907] Sum of Subarray Minimums
 */

// @lc code=start
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let max = 10i64.pow(9)+7;
        let mut ans = 0i64;
        let mut dot = 0;
        let mut s = Vec::<(i64,i64)>::new();
        for &a in &arr{
            let a = a as i64;
            let mut count =1;
            while !s.is_empty() && s.last().unwrap().0>=a{
                let (v,c)=s.pop().unwrap();
                count+=c;
                dot-=(v%max)*(c%max);
            }
            s.push((a,count));
            dot +=(a%max)*(count%max);
            ans+=dot%max;
        }
        (ans%max) as _
    }
}
// @lc code=end

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n=arr.len();
        let mut dp=vec![0;n];
        let mut ans=0;
        let mut s=Vec::new();
        for (i,&x) in arr.iter().enumerate(){
            while !s.is_empty() && arr[*s.last().unwrap()]>x{
                s.pop();
            }
            let k=if let Some(&j)=s.last(){i-j}else{i+1};
            dp[i]=(k as i64)*x as i64+if s.is_empty(){0}else{dp[i-k]};
            ans=(ans+dp[i])%1_000_000_007;
            s.push(i);
        }
        ans as _
    }
}