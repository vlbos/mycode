/*
 * @lc app=leetcode id=1130 lang=rust
 *
 * [1130] Minimum Cost Tree From Leaf Values
 */

// @lc code=start
impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut s = Vec::new();
        let mut ans = 0;
        let mut arr=arr;
        for i in 0..n{
            while !s.is_empty() && (*s.last().unwrap()<=arr[i]||i==n-1){
                let b = s.pop().unwrap();
                ans+=
                if !s.is_empty(){
                    arr[i].min(*s.last().unwrap())*b
                }else{
                    arr[i]*b
                };
                arr[i]=arr[i].max(b);
            }
            s.push(arr[i]);
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let n=arr.len();
        let mut dp=vec![vec![i32::MAX/4;n];n];
        let mut mval=vec![vec![0;n];n];
        for j in 0..n{
            mval[j][j]=arr[j];
            dp[j][j]=0;
            for i in (0..j).rev(){
                mval[i][j]=arr[i].max(mval[i+1][j]);
                for k in i..j{
                    dp[i][j]=dp[i][j].min(dp[i][k]+dp[k+1][j]+mval[i][k]*mval[k+1][j]);
                }
            }
        }
        dp[0][n-1]
    }
}