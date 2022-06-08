/*
 * @lc app=leetcode id=2101 lang=rust
 *
 * [2101] Detonate the Maximum Bombs
 */

// @lc code=start
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let is_connected=|u:usize,v:usize|->bool{
            let (dx,dy)=((bombs[u][0]-bombs[v][0]) as i64,(bombs[u][1]-bombs[v][1]) as i64);
            let r = bombs[u][2] as i64;
            r*r>=dx*dx+dy*dy
        };
        let n =bombs.len();
        let mut e=std::collections::HashMap::new();
        for i in 0..n{
        for j in 0..n{
        if i!=j && is_connected(i,j){
            e.entry(i).or_insert(Vec::new()).push(j);
        }}}
        let mut ans = 0;
        for i in 0..n{
                let mut cnt=1;
                let mut visited=vec![false;n];
                let mut q = std::collections::VecDeque::new();
                q.push_back(i);
                visited[i]=true;
                while let Some(j)=q.pop_front(){
                    for &ni in e.get(&j).unwrap_or(&Vec::new()){
                        if visited[ni]{
                        continue;
                        }
                        cnt+=1;
                         q.push_back(ni);
                            visited[ni]=true;
                    }
                }
                ans=ans.max(cnt);
        }
        ans
    }
}
// @lc code=end

