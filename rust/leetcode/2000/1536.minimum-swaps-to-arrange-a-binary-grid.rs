/*
 * @lc app=leetcode id=1536 lang=rust
 *
 * [1536] Minimum Swaps to Arrange a Binary Grid
 */

// @lc code=start
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut pos = vec![-1; n];
        for (i, v) in grid.iter().enumerate() {
             if let Some(j)=v.iter().rposition(|x| *x == 1){
                pos[i] =j as i32;
             }
        }
            let mut ans = 0;

        for i in 0..n {
            let mut k = -1;
            for j in i..n {
                if pos[j] <= i as i32 {
                    ans += (j - i) as i32;
                    k = j as i32;
                    break;
                }
            }
            if k == -1 {
                return -1;
            }
            for j in (i + 1..=k as usize).rev() {
                pos.swap(j, j - 1);
            }
        }
        ans
    }
}
// @lc code=end
