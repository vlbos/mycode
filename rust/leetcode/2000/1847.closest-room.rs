/*
 * @lc app=leetcode id=1847 lang=rust
 *
 * [1847] Closest Room
 */

// @lc code=start
impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut events = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            events.push(vec![0, room[1], room[0], i as i32]);
        }
        for (i, query) in queries.iter().enumerate() {
            events.push(vec![1, query[1], query[0], i as i32]);
        }
        events.sort_by(|a, b| {
            if a[1] == b[1] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        let mut ans = vec![-1; queries.len()];
        let mut valid = std::collections::BTreeSet::new();

        for event in &events {
            if event[0] == 0 {
                valid.insert(event[2]);
                continue;
            }
            let mut dist = i32::MAX;
            let item = valid.range(event[2]..).next();
            if let Some(&i) = item {
                if i - event[2] < dist {
                    dist = i - event[2];
                    ans[event[3] as usize] = i;
                }
            }
            if item != valid.iter().next() {
                if let Some(&prev) = valid.range(..event[2]).next_back() {
                    if event[2] - prev <= dist {
                        ans[event[3] as usize] = prev;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
