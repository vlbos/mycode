// 1724\. Checking Existence of Edge Length Limited Paths II
// =========================================================

// An undirected graph of `n` nodes is defined by `edgeList`,
// where `edgeList[i] = [ui, vi, disi]` denotes an edge between nodes `ui` and `vi` with distance `disi`.
//  Note that there may be **multiple** edges between two nodes, and the graph may not be connected.

// Implement the `DistanceLimitedPathsExist` class:

// *   `DistanceLimitedPathsExist(int n, int[][] edgeList)` Initializes the class with an undirected graph.
// *   `boolean query(int p, int q, int limit)` Returns `true` if there exists a path from `p` to `q` such that each edge on the path has a distance **strictly less than** `limit`,
// and otherwise `false`.

// **Example 1:**

// **![](https://assets.leetcode.com/uploads/2021/01/05/messed.png)**

// **Input**
// \["DistanceLimitedPathsExist", "query", "query", "query", "query"\]
// \[\[6, \[\[0, 2, 4\], \[0, 3, 2\], \[1, 2, 3\], \[2, 3, 1\], \[4, 5, 5\]\]\], \[2, 3, 2\], \[1, 3, 3\], \[2, 0, 3\], \[0, 5, 6\]\]
// **Output**
// \[null, true, false, true, false\]

// **Explanation**
// DistanceLimitedPathsExist distanceLimitedPathsExist = new DistanceLimitedPathsExist(6, \[\[0, 2, 4\], \[0, 3, 2\], \[1, 2, 3\], \[2, 3, 1\], \[4, 5, 5\]\]);
// distanceLimitedPathsExist.query(2, 3, 2); // return true. There is an edge from 2 to 3 of distance 1, which is less than 2.
// distanceLimitedPathsExist.query(1, 3, 3); // return false. There is no way to go from 1 to 3 with distances **strictly** less than 3.
// distanceLimitedPathsExist.query(2, 0, 3); // return true. There is a way to go from 2 to 0 with distance < 3: travel from 2 to 3 to 0.
// distanceLimitedPathsExist.query(0, 5, 6); // return false. There are no paths from 0 to 5.

// `**Constraints:**`

// *   `2 <= n <= 104`
// *   `0 <= edgeList.length <= 104`
// *   `edgeList[i].length == 3`
// *   `0 <= ui, vi, p, q <= n-1`
// *   `ui != vi`
// *   `p != q`
// *   `1 <= disi, limit <= 109`
// *   At most `104` calls will be made to `query`.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// public DistanceLimitedPathsExist(int n, int[][] edgeList) {

//     public boolean query(int p, int q, int limit)

// use std::collections::{HashMap, HashSet, VecDeque};
// #[allow(dead_code)]
// pub struct DistanceLimitedPathsExist {
//     g: HashMap<i32, Vec<(i32, i32)>>,
// }
// impl DistanceLimitedPathsExist {
//     pub fn new(_n: i32, edge_list: Vec<Vec<i32>>) -> Self {
//         let mut g = HashMap::new();
//         for e in &edge_list {
//             g.entry(e[0]).or_insert(Vec::new()).push((e[1], e[2]));
//             g.entry(e[1]).or_insert(Vec::new()).push((e[0], e[2]));
//         }
//         Self { g }
//     }
//     pub fn query(&self, p: i32, q: i32, limit: i32) -> bool {
//         let g = &self.g;
//         let mut que = VecDeque::from([p]);
//         let mut visited = HashSet::new();
//         let mut dis = 0;
//         while let Some(u) = que.pop_front() {
//             for &(v, d) in g.get(&u).unwrap_or(&Vec::new()) {
//                 if visited.contains(&v) {
//                     continue;
//                 }
//                 if dis + d >= limit {
//                     continue;
//                 }
//                 if v == q {
//                     return true;
//                 }
//                 dis += d;
//                 visited.insert(v);
//                 que.push_back(v);
//             }
//         }
//         false
//     }
// }


use std::collections::BTreeMap;
#[allow(dead_code)]
pub struct UnionFind {
     id: Vec<BTreeMap<i32, i32>>,
}
impl UnionFind {
    fn new(n:i32)->Self{
        Self{id:(0..n).map(|x|BTreeMap::from([(0,x)])).collect()}
    }
    fn unite(&mut self,u:i32,v:i32,limit:i32){
        let (i,j)=(self.find(u,limit),self.find(v,limit));
        if i==j{
        return}
        self.id[i as usize].insert(limit,j);
    }
    fn find(&mut self,u:i32,limit:i32)->i32{
        let i = *self.id[u as usize].range(..=limit).next_back().unwrap().1;
        if i==u{
        return u
        }
        let j = self.find(i,limit);
        self.id[u as usize].insert(limit,j);
        j
    }
}
#[allow(dead_code)]
pub struct DistanceLimitedPathsExist {
     uf:UnionFind,
}
impl DistanceLimitedPathsExist {
    pub fn new(n: i32, mut edge_list: Vec<Vec<i32>>) -> Self {
        let mut uf=UnionFind::new(n);
        edge_list.sort_by_key(|x|x[2]);
        for e in &edge_list{
            uf.unite(e[0],e[1],e[2]);
        }
        Self{uf}
    }

    pub fn query(&mut self, p: i32, q: i32, limit: i32) -> bool {
        self.uf.find(p,limit-1)==self.uf.find(q,limit-1)
    }
    
}

// use std::collections::BTreeMap;
// #[allow(dead_code)]
// pub struct DistanceLimitedPathsExist {
//      g: BTreeMap<i32, Vec<i32>>,
// }
// impl DistanceLimitedPathsExist {
//     pub fn new(n: i32, edge_list: Vec<Vec<i32>>) -> Self {
//         let mut v:Vec<i32>= (0..n).collect();
//         let mut el= edge_list;
//         el.sort_by_key(|x|x[2]);
//         let mut g =BTreeMap::new();
//         for e in &el {
//             let (x,y)=(Self::find(e[0],&mut v),Self::find(e[1],&mut v));
//             v[x as usize]=y;
//             g.insert(e[2],v.clone());
//         }
//         Self { g}
//     }

//     pub fn query(&mut self, p: i32, q: i32, limit: i32) -> bool {
//         if  self.g.range(..limit).count()<2{
//         return false
//         }
//         let i =*self.g.range(..limit).next_back().unwrap().0;
//         let mut v = self.g.get_mut(&i).unwrap();
//         Self::find(p,&mut v)== Self::find(q,&mut v)
//     }
//     pub fn find(mut x:i32,v:&mut Vec<i32>)->i32{
//                 while  v[x as usize]!=x{
//                      v[x as usize]= v[v[x as usize] as usize];
//                     x= v[x as usize];
//                 }
//             x
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_check_equivalence_1() {
        let mut d = DistanceLimitedPathsExist::new(
            6,
            vec![
                vec![0, 2, 4],
                vec![0, 3, 2],
                vec![1, 2, 3],
                vec![2, 3, 1],
                vec![4, 5, 5],
            ],
        );
        let queries = vec![vec![2, 3, 2], vec![1, 3, 3], vec![2, 0, 3], vec![0, 5, 6]];
        let expected_result = [true, false, true, false];
        for (q, &e) in queries.iter().zip(&expected_result) {
            assert_eq!(e, d.query(q[0], q[1], q[2]));
        }
    }
}
