/*
 * @lc app=leetcode id=753 lang=rust
 *
 * [753] Cracking the Safe
 */

// @lc code=start
impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        use std::collections::HashSet;
        fn dfs(node: i32, highest: i32, k: i32, seen: &mut HashSet<i32>, ans: &mut String) {
            for x in 0..k {
                let nei = node * 10 + x;
                if !seen.contains(&nei) {
                    seen.insert(nei);
                    dfs(nei % highest, highest, k, seen, ans);
                    ans.push((b'0' + x as u8) as char);
                }
            }
        }
        let highest = 10i32.pow(n as u32 - 1);
        let mut ans = String::new();
        let mut seen = HashSet::new();
        dfs(0, highest, k, &mut seen, &mut ans);
        ans.push_str("0".repeat(n as usize-1).as_str());
        ans
    }
}
// @lc code=end
