/*
 * @lc app=leetcode id=752 lang=rust
 *
 * [752] Open the Lock
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let first = "0000".to_string();
        if target == first {
            return 0;
        }
        let deadendsset = deadends.iter().collect::<HashSet<_>>();
        if deadendsset.contains(&first) {
            return -1;
        }
        let nums_prev = |x: u8| -> u8 {
            if x == b'0' {
                b'9'
            } else {
                x - 1
            }
        };
        let nums_succ = |x: u8| -> u8 {
            if x == b'9' {
                b'0'
            } else {
                x + 1
            }
        };
        let get = |nums: &String| -> Vec<String> {
            let mut res = Vec::new();
            let mut v = nums.bytes().collect::<Vec<u8>>();
            for i in 0..v.len() {
                let vi = v[i];
                v[i] = nums_prev(vi);
                res.push(String::from_utf8(v.clone()).unwrap());
                v[i] = nums_succ(vi);
                res.push(String::from_utf8(v.clone()).unwrap());
                v[i]=vi;
            }
            res
        };
        let mut seen = HashSet::new();
        let mut q = std::collections::VecDeque::new();
        seen.insert(first.clone());
        q.push_back((first, 0));
        while let Some(qe) = q.pop_front() {
            let (s, step) = qe;
            let ss = get(&s);
            for ns in &ss {
                if !seen.contains(ns) && !deadendsset.contains(ns) {
                    if *ns == target {
                        return step + 1;
                    }
                    q.push_back((ns.clone(), step + 1));
                    seen.insert(ns.clone());
                }
            }
        }
        -1
    }
}
// @lc code=end
