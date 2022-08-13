// 305\. Number of Islands II
// ==========================

// A 2d grid map of `m` rows and `n` columns is initially filled with water.
// We may perform an _addLand_ operation which turns the water at position (row, col) into a land.
// Given a list of positions to operate, **count the number of islands after each _addLand_ operation**.
// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
//  You may assume all four edges of the grid are all surrounded by water.

// **Example:**

// **Input:** m = 3, n = 3, positions = \[\[0,0\], \[0,1\], \[1,2\], \[2,1\]\]
// **Output:** \[1,1,2,3\]

// **Explanation:**

// Initially, the 2d grid `grid` is filled with water. (Assume 0 represents water and 1 represents land).

// 0 0 0
// 0 0 0
// 0 0 0

// Operation #1: addLand(0, 0) turns the water at grid\[0\]\[0\] into a land.

// 1 0 0
// 0 0 0   Number of islands = 1
// 0 0 0

// Operation #2: addLand(0, 1) turns the water at grid\[0\]\[1\] into a land.

// 1 1 0
// 0 0 0   Number of islands = 1
// 0 0 0

// Operation #3: addLand(1, 2) turns the water at grid\[1\]\[2\] into a land.

// 1 1 0
// 0 0 1   Number of islands = 2
// 0 0 0

// Operation #4: addLand(2, 1) turns the water at grid\[2\]\[1\] into a land.

// 1 1 0
// 0 0 1   Number of islands = 3
// 0 1 0

// **Follow up:**

// Can you do it in time complexity O(k log mn), where k is the length of the `positions`?

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
// pub  struct  UnionFind {
//     count: usize,
//     sz: Vec<usize>,
//     id: Vec<usize>,
//     land: Vec<bool>,
// }

// impl UnionFind {
//     pub fn   new(size: usize) -> UnionFind {
//         UnionFind {
//             count: 0,
//             sz: vec![1usize; size],
//             id: (0usize..size).collect::<Vec<usize>>(),
//             land: vec![false; size],
//         }
//     }

//     pub fn   add(&mut self, id: usize) {
//         if !self.land[id] {
//             self.land[id] = true;
//             self.count += 1;
//         }
//     }

//     pub fn   connected(&self, p: usize, q: usize) -> bool {
//         self.find(p) == self.find(q)
//     }

//     pub fn   find(&self, mut p: usize) -> usize {
//         while p != self.id[p] {
//             p = self.id[p]
//         }
//         p
//     }

//     pub fn   try_union(&mut self, p: usize, q: usize) {
//         if self.land[p] && self.land[q] {
//             let pid = self.find(p);
//             let qid = self.find(q);
//             if pid == qid {
//                 return;
//             }
//             if self.sz[pid] > self.sz[qid] {
//                 self.id[qid] = pid;
//                 self.sz[pid] += self.sz[qid];
//             } else {
//                 self.id[pid] = qid;
//                 self.sz[qid] += self.sz[pid];
//             }
//             self.count -= 1;
//         }
//     }

//     #[inline(always)]
//     pub fn   count(&self) -> usize {
//         self.count
//     }
// }

// const MOVEMENTS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
    // #[inline(always)]
    //pub fn  pos2id(x: usize, y: usize, _rows: usize, cols: usize) -> usize {
    //     x * cols + y
    // }

    pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        // let m = m as usize;
        // let n = n as usize;
        // let points = (m * n) as usize;
        // let mut counts = vec![];
        // let mut uf = UnionFind::new(points);
        // for pos in positions {
        //     let x = pos[0] as usize;
        //     let y = pos[1] as usize;
        //     let id = Solution::pos2id(x, y, m, n);
        //     uf.add(id);
        //     for movement in &MOVEMENTS {
        //         let next_x = x as isize + movement.0;
        //         let next_y = y as isize + movement.1;
        //         if next_x >= 0 && next_y >= 0 && next_x < (m as isize) && next_y < (n as isize) {
        //             let next_id = Solution::pos2id(next_x as usize, next_y as usize, m, n);
        //             uf.try_union(id, next_id);
        //         }
        //     }
        //     counts.push(uf.count() as i32);
        // }
        // counts
        use std::collections::HashMap;
        let mut ans = Vec::new();
        let mut count = 0;
        let mut map = HashMap::new();
        let dirs = [0, 1, 0, -1, 0];
        pub fn find(mut pos: i32, map: &mut HashMap<i32, i32>) -> i32 {
            let mut father = *map.get(&pos).unwrap();
            while father != *map.get(&father).unwrap() {
                father = *map.get(&father).unwrap();
            }
            while pos != *map.get(&pos).unwrap() {
                let temp = *map.get(&pos).unwrap();
                map.insert(pos, father);
                pos = temp;
            }
            father
        }
        for pos in &positions {
            let (x, y) = (pos[0], pos[1]);
            map.insert(x * n + y, x * n + y);
            count += 1;
            for (k, d) in dirs[1..].iter().enumerate() {
                let (nx, ny) = (x + dirs[k], y + d);
                if nx < 0 || nx >= m || ny < 0 || ny >= n || !map.contains_key(&(nx * n + ny)) {
                    continue;
                }
                let fa_x = find(x * n + y, &mut map);
                let fa_y = find(nx * n + ny, &mut map);
                if fa_x != fa_y {
                    map.insert(fa_x, fa_y);
                    count -= 1;
                }
            }
            ans.push(count);
        }
        ans
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_num_islands2() {
        let positions = [[0, 0], [0, 1], [1, 2], [2, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let expected = vec![1, 1, 2, 3];
        assert_eq!(Solution::num_islands2(3, 3, positions), expected);
    }
}
