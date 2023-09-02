/*
 * @lc app=leetcode id=765 lang=rust
 *
 * [765] Couples Holding Hands
 */

// @lc code=start
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len() / 2;
        let mut graph = vec![Vec::new(); n];
        row.chunks(2).for_each(|a| {
            let (l,r)=(a[0]as usize/2 ,a[1] as usize/2);
            if l != r {
                graph[l].push(r);
                graph[r].push(l);
            }
        });
        let mut visited = vec![false; n];
        let mut ans =0;
        for i in 0..n {
            if visited[i]  {
                continue;
            }
            visited[i] = true;
            let mut q = std::collections::VecDeque::new();
            q.push_back(i);
            let mut cnt = 0;
            while let Some(x) = q.pop_front() {
                cnt += 1;
                for &y in &graph[x] {
                    if !visited[y] {
                        visited[y] = true;
                        q.push_back(y);
                    }
                }
            }
            ans += cnt - 1;
        }
        ans
    }
}
// @lc code=end
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n=row.len();
        let mut parent:Vec<usize>=(0..n/2).collect();
        fn find(x:usize,parent:&mut Vec<usize>)->usize{
            let px=parent[x];
            if px!=x{
                parent[x]=find(px,parent);
            }
            parent[x]
        }
        let unite=|x:usize,y:usize,parent:&mut Vec<usize>|{
            let (px,py)=(find(x,parent),find(y,parent));
            if px!=py{
                parent[px]=py;
            }
        };
        for w in row.chunks(2){
            let (l,r)=(w[0] as usize/2,w[1] as usize/2);
            unite(l,r,&mut parent);
        }
        let mut m=std::collections::HashMap::new();
        for i in 0..n/2{
            *m.entry(find(i,&mut parent)).or_insert(0)+=1;
        }
        m.values().sum::<i32>()-m.len() as i32
    }
}