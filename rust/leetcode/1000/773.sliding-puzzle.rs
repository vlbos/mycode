/*
 * @lc app=leetcode id=773 lang=rust
 *
 * [773] Sliding Puzzle
 */

// @lc code=start
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let initial: Vec<i32> = board.iter().cloned().flatten().collect();
        let mut solved: Vec<i32> = (1..=5).collect();
        solved.push(0);
        if initial == solved {
            return 0;
        }
        let neighbors = [vec![1, 3], vec![0, 2, 4], vec![1, 5], vec![0, 4], vec![1, 3, 5], vec![2, 4]];
        let get = |mut status: Vec<i32>| -> Vec<Vec<i32>> {
            let mut ans = Vec::new();
            let x = status.iter().position(|&a| a == 0).unwrap();
            for &y in &neighbors[x] {
                status.swap(x, y);
                ans.push(status.clone());
                status.swap(x, y);
            }
            ans
        };
        let mut q = std::collections::VecDeque::new();
        q.push_back((initial.clone(), 0));
        let mut seen = std::collections::HashSet::new();
        seen.insert(initial.clone());
        while let Some((status, step)) = q.pop_front() {
            for next_status in get(status) {
                if seen.contains(&next_status) {
                    continue;
                }
                if next_status == solved {
                    return step + 1;
                }
                seen.insert(next_status.clone());
                q.push_back((next_status, step + 1));
            }
        }
        -1
    }
}
// @lc code=end
