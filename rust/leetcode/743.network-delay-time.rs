/*
 * @lc app=leetcode id=743 lang=rust
 *
 * [743] Network Delay Time
 */

// @lc code=start
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let inf = i32::MAX/2;
        let n = n as usize;
        let k  = k as usize;
        let  mut g = vec![vec![inf;n];n];
        for t in &times{
            let x = t[0] as usize-1;
            let y = t[1] as usize-1;
            g[x][y]=t[2];
        }
        let  mut dist = vec![inf;n];
        let mut used = vec![false;n];
        dist[k-1]=0;
        for i in 0..n{
            let  x = dist.iter().enumerate().filter(|(j,&y)|!used[*j]).min_by_key(|t|t.1).unwrap().0;
            used[x]=true;
            for y in 0..n{
                 dist[y]=dist[y].min(dist[x]+g[x][y]);
            }
        }
        let ans = *dist.iter().max().unwrap();
        if ans==inf{-1}else{ans}
    }
}
// @lc code=end

