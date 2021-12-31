/*
 * @lc app=leetcode id=77 lang=rust
 *
 * [77] Combinations
 */

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn back_track(
            n: i32,
            k: i32,
            idx: i32,
            mut ans: &mut Vec<Vec<i32>>,
            mut seq: &mut Vec<i32>,
        ) {
            if seq.len() == k as usize {
                ans.push(seq.clone());
                return;
            }
            if idx == n {
                return;
            }
            back_track(n, k, idx + 1,  &mut ans,  &mut seq);
            seq.push(idx+1);
            back_track(n, k, idx + 1,  &mut ans, &mut  seq);
            seq.pop();
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut seq = Vec::new();
        back_track(n, k, 0, &mut ans, &mut seq);
        ans
    }
}
// @lc code=end
