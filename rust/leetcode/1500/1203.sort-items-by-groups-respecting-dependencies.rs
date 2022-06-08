/*
 * @lc app=leetcode id=1203 lang=rust
 *
 * [1203] Sort Items by Groups Respecting Dependencies
 */

// @lc code=start
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (n as usize, m as usize);
        let mut group_item = vec![Vec::new(); n + m];
        let mut group_graph = vec![Vec::new(); n + m];
        let mut item_graph = vec![Vec::new(); n];
        let mut group_degree = vec![0; n + m];
        let mut item_degree = vec![0; n];
        let id: Vec<usize> = (0..n + m).collect();
        let mut left_id = m;
        let mut group = group;
        for i in 0..n {
            if group[i] == -1 {
                group[i] = left_id as i32;
                left_id += 1;
            }
            group_item[group[i] as usize].push(i);
        }
        for (i, &cur_group_id) in group.iter().enumerate() {
            for &item in &before_items[i] {
                let before_group_id = group[item as usize];
                if before_group_id == cur_group_id {
                    item_degree[i] += 1;
                    item_graph[item as usize].push(i);
                } else {
                    group_degree[cur_group_id as usize] += 1;
                    group_graph[before_group_id as usize].push(cur_group_id as usize);
                }
            }
        }
        let top_sort =
            |deg: &mut Vec<i32>, graph: &Vec<Vec<usize>>, items: &Vec<usize>| -> Vec<i32> {
                let mut q = std::collections::VecDeque::new();
                let mut ans = Vec::new();
                for &item in items {
                    if deg[item] == 0 {
                        q.push_back(item);
                    }
                }
                while let Some(u) = q.pop_front() {
                    ans.push(u as i32);
                    for &v in &graph[u] {
                        deg[v] -= 1;
                        if deg[v] == 0 {
                            q.push_back(v);
                        }
                    }
                }
                if ans.len() == items.len() {
                    ans
                } else {
                    Vec::new()
                }
            };
        let group_top_sort = top_sort(&mut group_degree, &group_graph, &id);
        if group_top_sort.is_empty() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        for &cur_group_id in &group_top_sort {
            if group_item[cur_group_id as usize].is_empty() {
                continue;
            }
            let res = top_sort(&mut item_degree, &item_graph, &group_item[cur_group_id as usize]);
            if res.is_empty() {
                return Vec::new();
            }
            ans.extend(res);
        }
        ans
    }
}
// @lc code=end
