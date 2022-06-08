/*
 * @lc app=leetcode id=1496 lang=rust
 *
 * [1496] Path Crossing
 */

// @lc code=start
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut s = std::collections::HashSet::new();
        let mut x = 0;
        let mut y = 0;
        s.insert(0);
        for c in path.chars() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                _ => x -= 1,
            };
            let n = x*20001+y;
            if   s.contains(&n) {
                return true;
            }else{
                s.insert(n);
            }
        }
        false
    }
}
// @lc code=end
