/*
 * @lc app=leetcode id=1345 lang=rust
 *
 * [1345] Jump Game IV
 */

// @lc code=start
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut idx_same_value = std::collections::HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            idx_same_value.entry(v).or_insert(Vec::new()).push(i);
        }
        let mut visited = std::collections::HashSet::from([0]);
        let mut q = std::collections::VecDeque::from([(0, 0)]);
        let n = arr.len();
        while let Some((idx, mut step)) = q.pop_front() {
            if idx == n - 1 {
                return step;
            }
            let v = arr[idx];
            step += 1;
            for &i in idx_same_value.get(&v).unwrap_or(&Vec::new()) {
                if !visited.contains(&i) {
                    visited.insert(i);
                    q.push_back((i, step));
                }
            }
            idx_same_value.remove(&v);
            if idx + 1 < n && !visited.contains(&(idx + 1)) {
                visited.insert(idx + 1);
                q.push_back((idx + 1, step));
            }
            if idx > 0 && !visited.contains(&(idx - 1)) {
                visited.insert(idx - 1);
                q.push_back((idx - 1, step));
            }
        }
        -1
    }
}
// @lc code=end
