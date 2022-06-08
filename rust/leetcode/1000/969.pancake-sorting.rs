/*
 * @lc app=leetcode id=969 lang=rust
 *
 * [969] Pancake Sorting
 */

// @lc code=start
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut sortedarr = arr
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        sortedarr.sort_by(|a, b| b.1.cmp(&a.1));
        let mut n = sortedarr.len() as i32;
        let mut ans = Vec::new();
        for &(i, _) in &sortedarr {
            let mut i = i as i32 + 1;
            for &f in &ans {
                if i <= f {
                    i = f + 1 - i;
                }
            }
            ans.extend(&[i, n]);
            n -= 1;
        }
        ans
    }
}
// @lc code=end
