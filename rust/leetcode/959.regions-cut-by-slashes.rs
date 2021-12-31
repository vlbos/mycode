/*
 * @lc app=leetcode id=959 lang=rust
 *
 * [959] Regions Cut By Slashes
 */

// @lc code=start
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        fn find(f:&mut Vec<usize>,x:usize)->usize{
            if f[x]==x{
            return x ;
            }
            let fa = find(f,f[x]);
            f[x]=fa;
            fa
        }
        let merge=|f:&mut Vec<usize>,x:usize,y:usize|{
            let fx = find(f,x);
            let fy = find(f,y);
            f[fx]=fy;
        };
        let n = grid.len();
        let mut f = (0..n*n*4).collect::<Vec<usize>>();
        for i in 0..n{
        for j in 0..n{
            let idx = i*n+j;
            if i<n-1{
                let bottom = idx+n;
                merge(&mut f,4*idx+2,4*bottom);
            }
            if j<n-1{
                let right = idx+1;
                merge(&mut f,4*idx+1,4*right+3);
            }
            if grid[i].chars().nth(j).unwrap()=='/' {
                 merge(&mut f,4*idx,4*idx+3);
                merge(&mut f,4*idx+1,4*idx+2);
            }else if grid[i].chars().nth(j).unwrap()=='\\' {
                merge(&mut f,4*idx,4*idx+1);
                merge(&mut f,4*idx+2,4*idx+3);
            }else{
                merge(&mut f,4*idx,4*idx+1);
                merge(&mut f,4*idx+1,4*idx+2);
                merge(&mut f,4*idx+2,4*idx+3);
            }
        }
        }
        let mut ans = std::collections::HashSet::new();
        for i in 0..n*n*4{
        let fa = find(&mut f,i);
        ans.insert(fa);
        }
        ans.len()  as _
    }
}
// @lc code=end

