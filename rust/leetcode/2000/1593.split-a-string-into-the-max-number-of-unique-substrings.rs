/*
 * @lc app=leetcode id=1593 lang=rust
 *
 * [1593] Split a String Into the Max Number of Unique Substrings
 */

// @lc code=start
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        use std::collections::HashSet;
        let mut ans = 1;
        fn back_tracking(
            bs: &[u8],
            a: &mut HashSet<Vec<u8>>,
            index: usize,
            split: i32,
            ans: &mut i32,
        ) {
            if index >= bs.len() {
                *ans = split.max(*ans);
                return;
            }
            for i in index..bs.len() {
                let ss = &bs[index..i + 1];
                if !a.contains(ss) {
                    a.insert(ss.to_vec());
                    back_tracking(bs, a, i + 1, split + 1, ans);
                    a.remove(ss);
                }
            }
        }
        let bs=s.as_bytes();
        let mut a = HashSet::new();
        back_tracking(bs, &mut a, 0, 0, &mut ans);
        ans
    }
}
// @lc code=end
