/*
 * @lc app=leetcode id=875 lang=rust
 *
 * [875] Koko Eating Bananas
 */

// @lc code=start
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let possible = |piles: &Vec<i32>, h: i32, k: i32| -> bool {
            let mut time = 0;
            for p in piles {
                time += (*p - 1) / k + 1;
            }
            time <= h
        };
        let mut lo = 1;
        let mut hi = 10i32.pow(9);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if !possible(&piles, h, mid) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
}
// @lc code=end
