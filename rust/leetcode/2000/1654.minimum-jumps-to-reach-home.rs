/*
 * @lc app=leetcode id=1654 lang=rust
 *
 * [1654] Minimum Jumps to Reach Home
 */

// @lc code=start
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        use std::collections::HashSet;
        let mut f: HashSet<i32> = forbidden.iter().cloned().collect();
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, false));
        while let Some((cur, cnt, used)) = q.pop_front() {
            if cur == x {
                return cnt;
            }
            if cur + a < 6000 && !f.contains(&(cur + a)) {
                f.insert(cur + a);
                q.push_back((cur + a, cnt + 1, false));
            }
            if !used && cur > b && !f.contains(&(cur - b)) {
                q.push_back((cur - b, cnt + 1, true));
            }
        }
        -1
    }
}
// @lc code=end
