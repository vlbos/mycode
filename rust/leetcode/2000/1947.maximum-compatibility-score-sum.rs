/*
 * @lc app=leetcode id=1947 lang=rust
 *
 * [1947] Maximum Compatibility Score Sum
 */

// @lc code=start
impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (students.len(), students[0].len());
        let mut g = vec![vec![0; m]; m];
        for i in 0..m {
            for j in 0..m {
                for k in 0..n {
                    if students[i][k] == mentors[j][k] {
                        g[i][j] += 1;
                    }
                }
            }
        }
        let m1 = 1 << m;
        let mut f = vec![0; m1];
        for mask in 1..m1 {
            let c = mask.count_ones() as usize;
            for i in 0..m {
                if mask & (1 << i) > 0 {
                    f[mask] = f[mask].max(f[mask ^ (1 << i)] + g[c - 1][i]);
                }
            }
        }
        f[m1 - 1]
    }
}
// @lc code=end
impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let m=students.len();
        let mut score=vec![vec![0;m];m];
        for (i,s) in students.iter().enumerate(){
            for (j,t) in mentors.iter().enumerate(){
                score[i][j]=s.iter().zip(t).filter(|(a,b)| a==b).count() as i32;
            }
        }
        fn dfs(i:usize,cur:i32,score:&Vec<Vec<i32>>,used:&mut Vec<bool>,ans:&mut i32){
                if i==score.len(){
                    *ans=cur.max(*ans);
                    return
                }
                for (j,&s) in score[i].iter().enumerate(){
                    if used[j]{
                        continue
                    }
                        used[j]=true;
                        dfs(i+1,cur+s,score,used,ans);
                        used[j]=false;
                }
        }
        let mut ans=0;
        let mut used=vec![false;m];
        dfs(0,0,&score,&mut used,&mut ans);
        ans
    }
}