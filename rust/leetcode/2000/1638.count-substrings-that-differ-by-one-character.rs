/*
 * @lc app=leetcode id=1638 lang=rust
 *
 * [1638] Count Substrings That Differ by One Character
 */

// @lc code=start
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let (ss, tt) = (s.as_bytes(), t.as_bytes());
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if ss[i] == tt[j] {
                    continue;
                }
                let mut l = 1;
                while i >= l && j >= l && ss[i - l] == tt[j - l] {
                    l += 1;
                }
                let mut r = 1;
                while i + r < m && j + r < n && ss[i + r] == tt[j + r] {
                    r += 1;
                }
                ans += l * r;
            }
        }
        ans as _
    }
}
// @lc code=end
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let (m,n)=(s.len(),t.len());
        let (mut dpl,mut dpr)=(vec![vec![0;n+1];m+1],vec![vec![0;n+1];m+1]);
        for (i,cs) in s.chars().enumerate(){
            for (j,ct) in t.chars().enumerate(){
                if cs==ct{
                    dpl[i+1][j+1]=dpl[i][j]+1;
                }
            }
        }
        for (i,cs) in s.bytes().enumerate().rev(){
            for (j,ct) in t.bytes().enumerate().rev(){
                if cs==ct{
                    dpr[i][j]=dpr[i+1][j+1]+1;
                }
            }
        }
        let mut ans=0;
        for (i,cs) in s.chars().enumerate(){
            for (j,ct) in t.chars().enumerate(){
                if cs!=ct{
                    ans+=(dpl[i][j]+1)*(dpr[i+1][j+1]+1);
                }
            }
        }
        ans
    }
}