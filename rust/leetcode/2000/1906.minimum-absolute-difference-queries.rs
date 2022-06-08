/*
 * @lc app=leetcode id=1906 lang=rust
 *
 * [1906] Minimum Absolute Difference Queries
 */

// @lc code=start
impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
         let c = 100;
        let n = nums.len();
        let mut pre = vec![vec![0; c + 1]];
        for (i, &num) in nums.iter().enumerate() {
            let p = pre[pre.len()-1].clone();
            pre.push(p);
            let m=pre.len()-1;
            pre[m][num as usize] += 1; 
        }
        let mut ans = Vec::new();
        for q in &queries {
            let mut last = 0;
            let mut best = i32::MAX;
            for j in 1..c + 1 {
                if pre[q[0] as usize][j] != pre[q[1] as usize + 1][j] {
                    if last != 0 {
                        best = best.min(j as i32 - last);
                    }
                    last = j as i32;
                }
            }
            if best == i32::MAX {
                best = -1;
            }
            ans.push(best);
        }
        ans
    }
}
// @lc code=end
