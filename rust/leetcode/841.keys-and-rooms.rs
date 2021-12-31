/*
 * @lc app=leetcode id=841 lang=rust
 *
 * [841] Keys and Rooms
 */

// @lc code=start
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut q = std::collections::VecDeque::new();
        let mut vis = std::collections::HashSet::new();
        vis.insert(0);
        q.push_back(0);
        while let Some(i) = q.pop_front() {
            for k in &rooms[i] {
                if vis.contains(k) {
                    continue;
                }
                vis.insert(*k);
                q.push_back((*k) as usize);
            }
        }
        vis.len() == rooms.len()
    }
}
// @lc code=end
