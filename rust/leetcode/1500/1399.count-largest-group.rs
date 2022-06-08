/*
 * @lc app=leetcode id=1399 lang=rust
 *
 * [1399] Count Largest Group
 */

// @lc code=start
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut r = vec![0; 37];
        let sum = |n: i32| -> usize {
            let mut nn = n;
            let mut d = 0;
            while nn > 0 {
                d += nn % 10;
                nn /= 10;
            }
            d as usize
        };
        for i in 1..=n {
            r[sum(i)] += 1;
        }
        let mut m = 0;
        let mut g = 0;
        for a in &r {
            if *a > m {
                m = *a;
                g = 1;
            } else if *a == m {
                g += 1;
            }
        }
        g
    }
}
// @lc code=end
