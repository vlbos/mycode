/*
 * @lc app=leetcode id=1001 lang=rust
 *
 * [1001] Grid Illumination
 */

// @lc code=start
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut points = std::collections::HashSet::new();
        use std::collections::HashMap;
        let (mut row, mut col, mut diagonal, mut antidiagonal) = (
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        );
        for l in &lamps {
            if points.contains(l) {
                continue;
            }
            points.insert(l.clone());
            *row.entry(l[0]).or_insert(0) += 1;
            *col.entry(l[1]).or_insert(0) += 1;
            *diagonal.entry(l[0] - l[1]).or_insert(0) += 1;
            *antidiagonal.entry(l[0] + l[1]).or_insert(0) += 1;
        }
        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            if *row.get(&q[0]).unwrap_or(&0)>0
                || *col.get(&q[1]).unwrap_or(&0)>0
                || *diagonal.get(&(q[0] - q[1])).unwrap_or(&0)>0
                || *antidiagonal.get(&(q[0] + q[1])).unwrap_or(&0)>0
            {
                ans[i] = 1;
            }
            for x in q[0] - 1..q[0] + 2 {
                for y in q[1] - 1..q[1] + 2 {
                    if x < 0 || y < 0 || x >= n || y >= n || !points.contains(&vec![x, y]) {
                        continue;
                    }
                    points.remove(&vec![x, y]);
                    row.entry(x).and_modify(|a| *a -= 1);
                    col.entry(y).and_modify(|a| *a -= 1);
                    diagonal.entry(x - y).and_modify(|a| *a -= 1);
                    antidiagonal.entry(x + y).and_modify(|a| *a -= 1);
                }
            }
        }
        ans
    }
}
// @lc code=end
