/*
 * @lc app=leetcode id=1007 lang=rust
 *
 * [1007] Minimum Domino Rotations For Equal Row
 */

// @lc code=start
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let check = |x: i32| -> i32 {
             let (mut ta, mut tb) = (0, 0);

            for (i, &t) in tops.iter().enumerate() {
                if t != x && bottoms[i] != x {
                    return -1;
                }
                if t != x {
                    ta += 1;
                } else if bottoms[i] != x {
                    tb += 1;
                }
            }
            ta.min(tb)
        };
        let ta = check(tops[0]);
        if ta != -1|| tops[0] == bottoms[0] {
            return ta;
        }
        check(bottoms[0])
    }
}
// @lc code=end
