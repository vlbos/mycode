/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len()<2{
        return s;
        }
        let sv = s.chars().collect::<Vec<char>>();
        let mut l = sv.len()-1;
        let mut r = 0;
        let mut max = 0;
        let mut start = 0;
        let mut i=0;
        while i<sv.len(){
            if sv.len()-i<=max/2{
            break;
            }
            l=i;
            r=i;
            while r+1<sv.len() && sv[r+1]==sv[r] {
                r+=1;
            }
            i=r+1;
            while r+1<sv.len() && l>0 && sv[l-1]==sv[r+1]{
                l-=1;
                r+=1;
            }
            if r-l+1>max{
                start = l;
                max=r-l+1;
            }
        }
        sv[start..start+max].iter().collect()
    }
}
// @lc code=end

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n=s.len();
        if n<2{
            return s
        }
        let mut dp=vec![vec![false;n];n];
        for i in 0..n{
            dp[i][i]=true;
        }
        let bs=s.as_bytes();
        let (mut begin,mut end)=(0,0);
        for len in 2..=n{
            for i in 0..n{
                let j=i+len-1;
                if j>=n{
                    break
                }
                dp[i][j]=if bs[i]==bs[j]{ if j-i<3{true}else{dp[i+1][j-1]}}else{false};
                if dp[i][j] && j-i>end-begin{
                    begin=i;
                    end=j;
                }
            }
        }
        s[begin..=end].to_string()
    }
}