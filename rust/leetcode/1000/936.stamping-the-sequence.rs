/*
 * @lc app=leetcode id=936 lang=rust
 *
 * [936] Stamping The Sequence
 */

// @lc code=start
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let (m, n) = (stamp.len(), target.len());
        let mut q = std::collections::VecDeque::new();
        use std::collections::HashSet;
        let mut ans = Vec::new();
        let mut done = vec![false; n];
        let mut a = std::collections::HashMap::new();
        let (bs, bt) = (stamp.as_bytes(), target.as_bytes());
        for i in 0..=n - m {
            let mut made = HashSet::new();
            let mut todo = HashSet::new();
            for j in 0..m {
                if bt[i + j] == bs[j] {
                    made.insert(i + j);
                } else {
                    todo.insert(i + j);
                }
            }
            if !made.is_empty(){
                a.insert(i,(made, todo.clone()));
            }
           
            if todo.is_empty() {
                ans.push(i as i32);
                for j in i..i + m {
                    if !done[j] {
                        q.push_back(j);
                        done[j] = true;
                    }
                }
            }
        }
        while let Some(i) = q.pop_front() {
            for j in if i > m - 1 { i - m + 1 } else { 0 }..=i.min(n - m) {
                if !a.contains_key(&j) {
                    continue;
                }
                if !a.get(&j).unwrap().1.contains(&i) {
                    continue;
                }
                a.get_mut(&j).unwrap().1.remove(&i);
                if !a.get(&j).unwrap().1.is_empty() {
                    continue;
                }
                ans.push(j as i32);
                for &k in &a.get(&j).unwrap().0 {
                    if !done[k] {
                        q.push_back(k);
                        done[k] = true;
                    }
                }
            }
        }
        if done.iter().any(|x| !*x) {
            return Vec::new();
        }
        ans.iter().cloned().rev().collect()
    }
}
// @lc code=end
