/*
 * @lc app=leetcode id=685 lang=rust
 *
 * [685] Redundant Connection II
 */

// @lc code=start
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
         let n = edges.len();
        let mut ancester = (0..=n).collect::<Vec<usize>>();
        fn find(index: usize, ancester: &mut Vec<usize>) -> usize {
            if index == ancester[index] {
                return index;
            }
            ancester[index] = find(ancester[index], ancester);
            ancester[index]
        }
        let merge = |u: usize, v: usize, ancester: &mut Vec<usize>| {
            let uindex = find(u, ancester);
            ancester[uindex] = find(v, ancester);
        };
        let mut parent = (0..=n).collect::<Vec<usize>>();
        let mut conflict = -1;
        let mut cycle = -1;
        for (i, edge) in edges.iter().enumerate() {
            let (node1, node2) = (edge[0] as usize, edge[1] as usize);
            if parent[node2] != node2 {
                conflict = i as i32;
            } else {
                parent[node2] = node1;
                if find(node1, &mut ancester) == find(node2, &mut ancester) {
                    cycle = i as i32;
                } else {
                    merge(node1, node2, &mut ancester);
                }
            }
        }
        if conflict < 0 {
            return edges[cycle as usize].clone();
        }
        if cycle < 0 {
            return edges[conflict as usize].clone();
        }
        let conflict_edge = &edges[conflict as usize];
        vec![parent[conflict_edge[1] as usize] as i32, conflict_edge[1]]
    }
}
// @lc code=end
