/*
 * @lc app=leetcode id=1240 lang=rust
 *
 * [1240] Tiling a Rectangle with the Fewest Squares
 */

// @lc code=start
impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let (n, m) = (n as usize, m as usize);
        let mut f = vec![vec![i32::MAX/3; m + 1]; n + 1];
        for i in 0..=n {
            f[i][0] = 0;
        }
        for i in 0..=m {
            f[0][i] = 0;
        }
        for i in 1..=n {
            for j in 1..=m {
                for k in 1..=i.min(j) {
                    let (i2, j1) = (i - k, j - k);
                    for i1 in 0..=i2 {
                        for j2 in j1..=j {
                            f[i][j] = f[i][j].min(
                                f[i1][j2] + f[i2][j - j2] + f[i - i1][j1] + f[i2 - i1][j2 - j1] + 1
                            );
                        }
                    }
                }
            }
        }
        f[n][m]
    }
}
// @lc code=end
impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        if n==m{
            return 1
        }
        let mut ans=n*m;

        let (n,m)=(n as usize, m as usize);
        let mut covered=vec![vec![false;m];n];
        fn dfs(cur:i32,covered:&mut Vec<Vec<bool>>,ans:&mut i32){
            if cur>=*ans{
                return
            }
            let (n,m)=(covered.len(), covered[0].len());
            let (mut r,mut c)=(n,m);
            for (i,row) in covered.iter().enumerate(){
                if let Some(j)=row.iter().position(|&x|!x){
                    r=i;
                    c=j;
                    break
                }
            }
            if r==n {
                *ans=(*ans).min(cur);
                return 
            }
            for len in (1..=(n-r).min(m-c)).rev(){
                let mut is_empty=true;
                 for (i,row) in covered[r..r+len].iter().enumerate(){
                        if row[c..c+len].iter().any(|&x|x){
                            is_empty=false;
                            break
                        }
                 }
                 if !is_empty{
                     continue
                 }
                 for i in r..r+len{
                     for j in c..c+len{
                         covered[i][j]=true;
                     }
                 }  
                 dfs(cur+1,covered,ans);
                 for i in r..r+len{
                     for j in c..c+len{
                         covered[i][j]=false;
                     }
                 } 
            }
        }
        dfs(0,&mut covered,&mut ans);
        ans
    }
}