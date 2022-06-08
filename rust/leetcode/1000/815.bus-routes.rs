/*
 * @lc app=leetcode id=815 lang=rust
 *
 * [815] Bus Routes
 */

// @lc code=start
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
       if source == target {
            return 0;
        }
        let n = routes.len();
        let mut edge:Vec<Vec<bool>> = vec![vec![false; n]; n];
        use  std::collections::HashMap;
        let mut rec:HashMap<i32,Vec<usize>> = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for &site in route {
                for &j in rec.get(&site).unwrap_or(&Vec::new()) {
                    edge[i][j] = true;
                    edge[j][i] = true;
                }
                rec.entry(site).or_insert(Vec::new()).push(i);
            }
        }
        let mut dis = vec![-1; n];
        let mut q = std::collections::VecDeque::new();
        for &bus in rec.get(&source).unwrap_or(&Vec::new()) {
            dis[bus] = 1;
            q.push_back(bus);
        }
        while let Some(x) = q.pop_front() {
            for y in 0..n {
                if edge[x][y] && dis[y] == -1 {
                    dis[y] = dis[x] + 1;
                    q.push_back(y);
                }
            }
        }
        let ans = rec
            .get_mut(&target)
            .unwrap_or(&mut Vec::new()).iter_mut()
            .fold(i32::MAX, |ret, &mut val| ret.min(dis[val]));
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
