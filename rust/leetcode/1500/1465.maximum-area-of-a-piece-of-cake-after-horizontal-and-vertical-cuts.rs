/*
 * @lc app=leetcode id=1465 lang=rust
 *
 * [1465] Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
 */

// @lc code=start
impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let inf=1000_000_007;
        let mut horizontal_cuts=horizontal_cuts;
        let mut vertical_cuts=vertical_cuts;
        horizontal_cuts.push(h);
        vertical_cuts.push(w);
        horizontal_cuts.sort();
        vertical_cuts.sort();
        let (mut mr,mut mc)=(0,0);
        let (mut nr,mut nc)=(0,0);
        for &h in &horizontal_cuts{
            mr=mr.max(h-nr);
            nr=h;
        }
        for &v in &vertical_cuts{
            mc=mc.max(v-nc);
            nc=v;
        }
        ((mr as i64*mc as i64)%inf) as _
    }
}
// @lc code=end

