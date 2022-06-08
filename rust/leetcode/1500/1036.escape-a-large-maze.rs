/*
 * @lc app=leetcode id=1036 lang=rust
 *
 * [1036] Escape a Large Maze
 */

// @lc code=start
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
       if blocked.len() < 2 {
            return true;
        }
        use std::collections::HashSet;
        let blocked: HashSet<Vec<i32>> = blocked.into_iter().collect();
        let bn = blocked.len() as i32;
        let dirs = [0, 1, 0, -1, 0];
        let boundry = 1_000_000;
        let check = |source: &Vec<i32>, target: &Vec<i32>| -> i32 {
            let mut countdown = bn * (bn - 1) / 2;
            let mut q = std::collections::VecDeque::new();
            q.push_back(source.clone());
            let mut visited = HashSet::new();
            visited.insert(source.clone());
            while let Some(xy) = q.pop_front() {
                if countdown <= 0 {
                    break;
                }
                for i in 0..dirs.len() - 1 {
                    let (nx, ny) = (xy[0] + dirs[i], xy[1] + dirs[i + 1]);
                    let nxy=vec![nx, ny];
                    if nx < 0 || nx >= boundry || ny < 0 || ny >= boundry ||blocked.contains(&nxy) || visited.contains(&nxy){
                        continue;
                    }
                    if nx == target[0] && ny == target[1] {
                        return 1;
                    }
                    countdown -= 1;
                    q.push_back(nxy.clone());
                    visited.insert(nxy);
                }
            }
            if countdown > 0 {
                return -1;
            }
            0
        };
        let mut result = check(&source, &target);
        if result == 1 {
            return true;
        }
        if result == -1 {
            return false;
        }
        result = check(&target, &source);
        if result == -1 {
            return false;
        }
        true
    }
}
// @lc code=end
