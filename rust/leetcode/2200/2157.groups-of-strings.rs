/*
 * @lc app=leetcode id=2157 lang=rust
 *
 * [2157] Groups of Strings
 */

// @lc code=start
impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        let mut wordmasks = std::collections::HashMap::new();
        for word in &words {
            let mut mask = 0;
            for b in word.bytes() {
                mask |= (1 << (b - b'a') as u32);
            }
            *wordmasks.entry(mask).or_insert(0) += 1;
        }
        let get_adjacent = |mask: i32| {
            let mut adj = Vec::new();
            for i in 0..26 {
                adj.push(mask ^ (1 << i));
            }
            for i in 0..26 {
                if mask & (1 << i) == 0 {
                    continue;
                }
                for j in 0..26 {
                    if mask & (1 << j) == 0 {
                        adj.push(mask ^ (1 << i) ^ (1 << j));
                    }
                }
            }

            adj
        };
        let mut used = std::collections::HashSet::new();
        let mut best = 0;
        let mut cnt = 0;
        for (&mask, &occ) in &wordmasks {
            if used.contains(&mask) {
                continue;
            }
            let mut q = std::collections::VecDeque::from([mask]);
            used.insert(mask);
            let mut total = occ;
            while let Some(u) = q.pop_front() {
                for v in get_adjacent(u) {
                    if used.contains(&v) {
                        continue;
                    }
                    if let Some(&w) = wordmasks.get(&v) {
                        q.push_back(v);
                        used.insert(v);
                        total += w;
                    }
                }
            }
            best = best.max(total);
            cnt += 1;
        }
        vec![cnt, best]
    }
}
// @lc code=end
