/*
 * @lc app=leetcode id=1489 lang=rust
 *
 * [1489] Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree
 */

// @lc code=start
struct UnionFind {
    parent: Vec<i32>,
    size: Vec<i32>,
    n: i32,
    set_count: i32,
}
impl UnionFind {
    fn new(n: i32) -> Self {
        Self {
            n,
            set_count: n,
            size: vec![1; n as usize],
            parent: (0..n).collect(),
        }
    }
    fn find_set(&mut self, x: i32) -> i32 {
        let xi = x as usize;
        if self.parent[xi] == x {
            return x;
        }
        self.parent[xi] = self.find_set(self.parent[xi]);
        self.parent[xi]
    }
    fn unite(&mut self, x: i32, y: i32) -> bool {
        let (x, y) = (self.find_set(x) as usize, self.find_set(y) as usize);
        if x == y {
            return false;
        }
        let (x, y) = if self.size[x] < self.size[y] {
            (y, x)
        } else {
            (x, y)
        };
        self.parent[y] = x as i32;
        self.size[x] += self.size[y];
        self.set_count -= 1;
        true
    }
    fn connected(&mut self, x: i32, y: i32) -> bool {
        self.find_set(x) == self.find_set(y)
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn tarjanscc(n: usize, edges: &Vec<Vec<i32>>, edge_ids: &Vec<Vec<i32>>) -> Vec<i32> {
            let mut low = vec![-1; n as usize];
            let mut dfn = vec![-1; n as usize];
            let mut ans = Vec::new();
            let mut ts = -1;
            fn get_cutting_edge(
                u: usize,
                parent_edge_id: i32,
                edges: &Vec<Vec<i32>>,
                edge_ids: &Vec<Vec<i32>>,
                ts: &mut i32,
                low: &mut Vec<i32>,
                dfn: &mut Vec<i32>,
                ans: &mut Vec<i32>,
            ) {
                *ts += 1;
                low[u] = *ts;
                dfn[u] = *ts;
                for (i, &v) in edges[u].iter().enumerate() {
                    let v = v as usize;
                    let id = edge_ids[u][i];
                    if dfn[v] == -1 {
                        get_cutting_edge(v, id, edges, edge_ids, ts, low, dfn, ans);
                        low[u] = low[u].min(low[v]);
                        if low[v] > dfn[u] {
                            ans.push(id);
                        }
                    } else if id != parent_edge_id {
                        low[u] = low[u].min(dfn[v]);
                    }
                }
            }
            for i in 0..n {
                if dfn[i] == -1 {
                    get_cutting_edge(
                        i, -1, edges, edge_ids, &mut ts, &mut low, &mut dfn, &mut ans,
                    );
                }
            }
            ans
        }
        let mut edges = edges;
        let m = edges.len();
        for i in 0..m {
            edges[i].push(i as i32);
        }
        edges.sort_by_key(|x| x[2]);
        let mut uf = UnionFind::new(n);
        let mut ans = vec![Vec::new(); 2];
        let mut label = vec![0; m];
        let mut i = 0;
        while i < m {
            let w = edges[i][2];
            let mut j = i;
            while j < m && edges[j][2] == edges[i][2] {
                j += 1;
            }
            let mut comp_to_id = std::collections::HashMap::new();
            let mut gn = 0;
            for k in i..j {
                let (x, y) = (uf.find_set(edges[k][0]), uf.find_set(edges[k][1]));
                if x != y {
                    if !comp_to_id.contains_key(&x) {
                        comp_to_id.insert(x, gn);
                        gn += 1;
                    }
                    if !comp_to_id.contains_key(&y) {
                        comp_to_id.insert(y, gn);
                        gn += 1;
                    }
                } else {
                    label[edges[k][3] as usize] = -1;
                }
            }
            let mut gm = vec![Vec::new(); gn];
            let mut gmid = vec![Vec::new(); gn];
            for k in i..j {
                let (x, y) = (uf.find_set(edges[k][0]), uf.find_set(edges[k][1]));
                if x != y {
                    let (idx, idy) = (*comp_to_id.get(&x).unwrap() as i32, *comp_to_id.get(&y).unwrap() as i32);
                    gm[idx as usize].push(idy);
                    gmid[idx as usize].push(edges[k][3]);
                    gm[idy as usize].push(idx);
                    gmid[idy as usize].push(edges[k][3]);
                }
            }
            let bridges = tarjanscc(gn, &gm, &gmid);
            for &id in &bridges {
                ans[0].push(id);
                label[id as usize] = 1;
            }
            for k in i..j {
                uf.unite(edges[k][0], edges[k][1]);
            }
            i = j;
        }
        for i in 0..m {
            if label[i] == 0 {
                ans[1].push(i as i32);
            }
        }
        ans
    }
}
// @lc code=end
