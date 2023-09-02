/*
 * @lc app=leetcode id=678 lang=rust
 *
 * [678] Valid Parenthesis String
 */

// @lc code=start
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut maxcount = 0;
        let mut mincount = 0;
        for c in s.chars() {
            if c == '(' {
                mincount += 1;
                maxcount += 1;
            } else if c == ')' {
                mincount = (mincount - 1).max(0);
                maxcount -= 1;
                if maxcount < 0 {
                    return false;
                }
            } else {
                mincount = (mincount - 1).max(0);
                maxcount += 1;
            }
        }
        mincount == 0
    }
}
// @lc code=end
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let n=s.len();
         let bs=s.as_bytes();
        if n==1{
            return bs[0]==b'*'
        }
        let mut dp=vec![vec![false;n];n];
        for (i,c) in s.chars().enumerate(){
            if c=='*'{
                dp[i][i]=true;
            }
        }
       
        for (i,w) in bs.windows(2).enumerate(){
            let (c1,c2)=(w[0],w[1]);
            dp[i][i+1]=(c1==b'(' || c1==b'*') && (c2==b')' || c2==b'*') ;
        }
        for i in (0..n-2).rev(){
            let c1=bs[i];
            for j in i+2..n{
                let c2=bs[j];
                if (c1==b'(' || c1==b'*') && (c2==b')' || c2==b'*'){
                    dp[i][j]=dp[i+1][j-1];
                }
                for k in i..j{
                    if dp[i][j]{
                        break
                    }
                    dp[i][j]=dp[i][k]&& dp[k+1][j];
                }
            }

        }
        dp[0][n-1]
    }
}