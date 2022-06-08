/*
 * @lc app=leetcode id=2092 lang=rust
 *
 * [2092] Find All People With Secret
 */

// @lc code=start
impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let m = meetings.len();
        let mut meetings = meetings;
        meetings.sort_by_key(|x| x[2]);
        let mut secret = vec![false; n as usize];
        secret[0] = true;
        secret[first_person as usize] = true;
        let mut vertices = std::collections::HashSet::new();
        let mut edges = std::collections::HashMap::new();
        let mut i = 0;
        while i < m {
            let mut j = i;
            while j + 1 < m && meetings[j + 1][2] == meetings[i][2] {
                j += 1;
            }
            vertices.clear();
            edges.clear();
            for meeting in &meetings[i..=j] {
                let (x, y) = (meeting[0] as usize, meeting[1] as usize);
                vertices.insert(x);
                vertices.insert(y);
                edges.entry(x).or_insert(Vec::new()).push(y);
                edges.entry(y).or_insert(Vec::new()).push(x);
            }
            let mut q = std::collections::VecDeque::from(
                vertices.iter().cloned().filter(|x| secret[*x]).collect::<Vec<usize>>(),
            );
            while let Some(u) = q.pop_front() {
                for &v in edges.get(&u).unwrap_or(&Vec::new()) {
                    if !secret[v] {
                        secret[v] = true;
                        q.push_back(v);
                    }
                }
            }
            i = j + 1;
        }
        secret
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(i, _)| i as i32)
            .collect()
    }
}
// @lc code=end
