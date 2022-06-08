/*
 * @lc app=leetcode id=854 lang=rust
 *
 * [854] K-Similar Strings
 */

// @lc code=start
impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
       let bs2 = s2.as_bytes();
        let neighbors = |s: &String| -> Vec<String> {
            let mut i = 0;
            for (j, b) in s.bytes().enumerate() {
                if b != bs2[j] {
                    i = j;
                    break;
                }
            }
            let bs = s.as_bytes();
            let mut t: Vec<char> = s.chars().collect();
            let mut ans = Vec::new();
            for j in i + 1..bs.len() {
                if bs[j] == bs2[i] {
                    t.swap(i, j);
                    ans.push(t.iter().cloned().collect());
                    t.swap(i, j);
                }
            }
            ans
        };
        let mut q = std::collections::VecDeque::new();
        q.push_back(s1.clone());
        let mut dist = std::collections::HashMap::new();
        dist.insert(s1.clone(), 0);
        while let Some(s) = q.pop_front() {
            if s == s2 {
                return *dist.get(&s).unwrap();
            }
            for t in neighbors(&s) {
                if !dist.contains_key(&t) {
                    dist.insert(t.clone(), *dist.get(&s).unwrap_or(&0) + 1);
                    q.push_back(t.clone());
                }
            }
        }
        0
    }
}
// @lc code=end
