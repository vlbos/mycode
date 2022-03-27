/*
 * @lc app=leetcode id=1306 lang=rust
 *
 * [1306] Jump Game III
 */

// @lc code=start
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let start = start as usize;
        if arr[start] == 0 {
            return true;
        }
        let n = arr.len();
        let mut used = vec![false; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(start);
        used[start] = true;
        while let Some(i) = q.pop_front() {
            let v = arr[i] as usize;
            if i + v < n && !used[i + v] {
                if arr[i + v] == 0 {
                    return true;
                }
                q.push_back(i + v);
                used[i + v] = true;
            }
            if i as i32 - arr[i] >= 0 && !used[i - v] {
                if arr[i - v] == 0 {
                    return true;
                }
                q.push_back(i - v);
                used[i - v] = true;
            }
        }
        false
    }
}
// @lc code=end
