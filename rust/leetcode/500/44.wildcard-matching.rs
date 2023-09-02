/*
 * @lc app=leetcode id=44 lang=rust
 *
 * [44] Wildcard Matching
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (sn, pn) = (s.len(), p.len());
        let (bs, bp) = (s.as_bytes(), p.as_bytes());
        let mut i = 0;
        while i < sn && i < pn && bp[pn - i - 1] != b'*' {
            let (u, v) = (bs[sn - i - 1], bp[pn - i - 1]);
            if u == v || v == b'?' {
                i += 1;
            } else {
                return false;
            }
        }
        if i == pn {
            return i == sn;
        }
        let (mut si, mut pi) = (0, 0);
        let (mut sr, mut pr) = (sn, pn);
        while si < sn && pi < pn {
            let (u, v) = (bs[si], bp[pi]);
            if v == b'*' {
                pi += 1;
                sr = si;
                pr = pi;
            } else if u == v || v == b'?' {
                si += 1;

                pi += 1;
            } else if sr != sn && sr + 1 < sn {
                sr += 1;
                si = sr;
                pi = pr;
            } else {
                return false;
            }
        }
        bp[pi..].iter().all(|x| *x == b'*')
    }
}
// @lc code=end
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (m,n)=(s.len(),p.len());
        let (bs,bp)=(s.as_bytes(),p.as_bytes());
        let mut dp=vec![vec![false;n+1];m+1];
        dp[0][0]=true;
        for i in 1..=n{
            if bp[i-1]==b'*'{
                dp[0][i]=true;
            }else{
                break
            }
        }
        for i in 1..=m{
            for j in 1..=n{
                if bp[j-1]==b'*'{
                dp[i][j]=dp[i][j-1]|dp[i-1][j];
            }else if bp[j-1]==b'?'||bp[j-1]==bs[i-1]{
                 dp[i][j]=dp[i-1][j-1];
            }
            }
        }
        dp[m][n]
    }
}