/*
 * @lc app=leetcode id=913 lang=rust
 *
 * [913] Cat and Mouse
 */

// @lc code=start
impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut degrees = vec![vec![vec![0, 0]; n]; n];
        let mut results = vec![vec![vec![0, 0]; n]; n];
        for i in 0..n {
            for j in 1..n {
                degrees[i][j][0] = graph[i].len();
                degrees[i][j][1] = graph[j].len();
            }
        }
        for &y in &graph[0] {
            for i in 0..n {
                degrees[i][y as usize][1] -= 1;
            }
        }
        let mut q = std::collections::VecDeque::new();
        for j in 1..n {
            results[0][j][0] = 1;
            results[0][j][1] = 1;
            q.push_back((0, j, 0));
            q.push_back((0, j, 1));
        }
        for i in 1..n {
            results[i][i][0] = 2;
            results[i][i][1] = 2;
            q.push_back((i, i, 0));
            q.push_back((i, i, 1));
        }
        while let Some((mouse, cat, turn)) = q.pop_front() {
            let result = results[mouse][cat][turn];
            let prev_states: Vec<(usize, usize, usize)> = if turn == 0 {
                graph[cat]
                    .iter()
                    .cloned()
                    .map(|prev| (mouse, prev as usize, 1))
                    .collect()
            } else {
                graph[mouse]
                    .iter()
                    .cloned()
                    .map(|prev| (prev as usize, cat, 0))
                    .collect()
            };
            for (prev_mouse, prev_cat, prev_turn) in prev_states {
                if prev_cat == 0 {
                    continue;
                }
                if results[prev_mouse][prev_cat][prev_turn] == 0 {
                    let can_win = result == 1 && prev_turn == 0 || result == 2 && prev_turn == 1;
                    if can_win {
                        results[prev_mouse][prev_cat][prev_turn] = result;
                        q.push_back((prev_mouse, prev_cat, prev_turn));
                    } else {
                        degrees[prev_mouse][prev_cat][prev_turn] -= 1;
                        if degrees[prev_mouse][prev_cat][prev_turn] == 0 {
                            results[prev_mouse][prev_cat][prev_turn] =
                                if prev_turn == 0 { 2 } else { 1 };
                            q.push_back((prev_mouse, prev_cat, prev_turn));
                        }
                    }
                }
            }
        }
        results[1][2][0]
    }
}
// @lc code=end
