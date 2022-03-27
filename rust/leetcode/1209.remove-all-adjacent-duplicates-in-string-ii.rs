/*
 * @lc app=leetcode id=1209 lang=rust
 *
 * [1209] Remove All Adjacent Duplicates in String II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut ans = s.chars().collect::<Vec<char>>();
        let mut j = 0;
        let mut counts = Vec::new();
        for i in 0..ans.len() {
            ans[j] = ans[i];
            if j == 0 || ans[j] != ans[j - 1] {
                counts.push(1);
            } else {
                if let Some(mut n) = counts.last_mut() {
                    *n += 1;
                    if *n == k {
                        j -= k as usize;
                        counts.pop();
                    }
                }
            }
            j += 1;
        }
        ans[..j].iter().collect()
    }
}
// @lc code=end
