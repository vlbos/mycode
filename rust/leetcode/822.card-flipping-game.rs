/*
 * @lc app=leetcode id=822 lang=rust
 *
 * [822] Card Flipping Game
 */

// @lc code=start
impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let vv = fronts
            .iter()
            .zip(backs.iter())
            .filter(|(a, b)| a == b)
            .map(|(a, _)| *a)
            .collect::<std::collections::HashSet<_>>();
        let fmin = fronts.iter().filter(|x| !vv.contains(x)).min();
        let bmin = backs.iter().filter(|x| !vv.contains(x)).min();
        match (fmin, bmin) {
            (Some(fm), Some(bm)) => *fm.min(bm),
            (Some(fm), None) => *fm,
            (None, Some(bm)) => *bm,
            _ => 0,
        }
    }
}
// @lc code=end
