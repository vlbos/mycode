/*
 * @lc app=leetcode id=1202 lang=rust
 *
 * [1202] Smallest String With Swaps
 */

// @lc code=start
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        if pairs.is_empty() {
            return s;
        }
        let n = s.len();
        let mut f = (0..n).into_iter().collect::<Vec<usize>>();
        let mut rank = vec![1; n];
        fn find(f: &mut Vec<usize>, x: usize) -> usize {
            if f[x] == x {
                return x;
            }
            f[x] = find(f, f[x]);
            f[x]
        }
        let merge = |f: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize| {
            let px = find(f, x);
            let py = find(f, y);
            if px == py {
                return;
            }
            let (x, y) = if rank[x] < rank[y] { (y, x) } else { (x, y) };
            rank[x] += rank[y];
            f[py] = px;
        };
        for p in &pairs {
            merge(&mut f, &mut rank, p[0] as usize, p[1] as usize);
        }
        let mut m = std::collections::HashMap::new();
        use std::cmp::Reverse;
        for (i, c) in s.chars().enumerate() {
            m.entry(find(&mut f, i))
                .or_insert(std::collections::BinaryHeap::new())
                .push(Reverse(c));
        }
        let mut ans = Vec::new();
        for i in 0..n {
            if let Some(mut cc) = m.get_mut(&find(&mut f, i)) {
                if let Some(Reverse(c)) = cc.pop() {
                    ans.push(c);
                }
            }
        }
        ans.iter().collect()
    }
}
// @lc code=end


impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n=s.len();
        let mut parent:Vec<usize>=(0..n).collect();
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
        for p in &pairs{
            unite(p[0] as usize,p[1] as usize,&mut parent);
        }
        let mut mp=std::collections::HashMap::new();
        for (i,c) in s.chars().enumerate(){
            mp.entry(find(i,&mut parent)).or_insert(Vec::new()).push(c);
        }
        for v in mp.values_mut(){
            v.sort_by(|a,b| b.cmp(&a));
        }
         (0..s.len()).map(|i|mp.get_mut(&find(i,&mut parent)).unwrap().pop().unwrap()).collect()
    }
}