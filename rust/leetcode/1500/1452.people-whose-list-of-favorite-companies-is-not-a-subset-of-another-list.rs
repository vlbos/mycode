/*
 * @lc app=leetcode id=1452 lang=rust
 *
 * [1452] People Whose List of Favorite Companies Is Not a Subset of Another List
 */

// @lc code=start
impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
       use std::collections::HashSet;
        let mut s = favorite_companies
            .iter()
            .enumerate()
            .map(|(j, y)| (j, y.iter().cloned().collect::<HashSet<_>>()))
            .collect::<Vec<(usize, HashSet<String>)>>();
        s.sort_by_key(|x| x.1.len());
        let mut ans = Vec::new();
        for (i, f) in s.iter().enumerate() {
            let mut flag = true;
            for j in (i + 1..s.len()) {
                if f.1.is_subset(&s[j].1) {
                    flag = false;
                    break;
                }
            }
            if flag {
                ans.push(f.0 as i32);
            }
        }
        ans.sort();
        ans
    }
}
// @lc code=end
