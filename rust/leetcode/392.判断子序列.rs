/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 * [392] 判断子序列
 */

// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // let mut p = s.into_bytes();
        // let mut q = t.into_bytes();
        // let mut i = 0;
        // let mut j = 0;
        // while i<p.len()&&j<q.len(){
        //      if p[i]==q[j]{
        //      i+=1;
        //     }
        //     j+=1;
        // }
        // i==p.len()
        let s = s.into_bytes();
        let t = t.into_bytes();
        let n =s.len();
        let m=t.len();
        let mut arr= vec![vec![0;26];m+1];
        for i in 0..26{
            arr[m][i]=m;
        }
        for i in 0..m{
            for j in 0..26{
                let k = (m-1-i) as usize;
                let jj = j as usize;
                if t[k]==(j+('a' as u8)){
                    arr[k][jj] =k;
                }
                else{
                    arr[k][jj]= arr[k+1][jj];
                }
            }
        }
        let mut add = 0;
        for i in 0..n{
            let k = (s[i]-('a' as u8)) as usize;
            if arr[add][k]==m{
            return false;
            }
            add = arr[add][k]+1;
        }
        true
    }
}
// @lc code=end

