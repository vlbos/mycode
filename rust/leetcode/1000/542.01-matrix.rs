/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 */

// @lc code=start
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut vis = vec![vec![false;n];m];
        let mut q = std::collections::VecDeque::new();
        for i in 0..m{
            for j in 0..n{
                     if  mat[i][j]==0{
                           q.push_back(vec![i,j]);
                            vis[i][j]=true;
                     }
            }
        }
        let mut mat = mat;
        let mm = mat.len() as i32;
        let nn = mat[0].len() as i32;
        let ds = vec![vec![-1,0],vec![1,0],vec![0,-1],vec![0,1]];
        while let Some(qq)=q.pop_front(){
            let i = qq[0];
            let j = qq[1];
            for xy in &ds{
                  let ii = i as i32+xy[0];
                  let jj = j as i32+xy[1];
                  if ii<0||ii==mm || jj<0||jj==nn{
                        continue;
                    }
                    let iii = ii as usize;
                    let jjj = jj as usize;
      
                    if  !vis[iii][jjj]{
                        mat[iii][jjj]=mat[i][j]+1;
                        q.push_back(vec![iii,jjj]);
                        vis[iii][jjj]=true;
                    }
            }
        }
       
        mat
    }
}
// @lc code=end

