/*
 * @lc app=leetcode id=216 lang=rust
 *
 * [216] Combination Sum III
 */

// @lc code=start
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if n < k {
            return Vec::new();
        }
        fn back_track(
            n: i32,
            sum: i32,
            k: usize,
            idx: i32,
            seq: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if seq.len() == k && sum == n {
                ans.push(seq.clone());
                return;
            }

            if seq.len() > k || idx > 9 {
                return;
            }
            seq.push(idx);
            back_track(n, sum + idx, k, idx + 1, seq, ans);
            seq.pop();
            back_track(n, sum, k, idx + 1, seq, ans);

        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut seq: Vec<i32> = Vec::new();
        back_track(n, 0, k as usize, 1, &mut seq, &mut ans);

        ans
    }
}
// @lc code=end
