/*
 * @lc app=leetcode id=2103 lang=rust
 *
 * [2103] Rings and Rods
 */

// @lc code=start
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let color = [b'R', b'G', b'B'];
        let br = rings.as_bytes();
        let n2 = rings.len();
        let mut r = vec![vec![0; 3]; 10];
        for i in (0..n2).step_by(2) {
            let j = color.iter().position(|x| *x == br[i]).unwrap();
            r[(br[i + 1] - b'0') as usize][j] += 1;
        }
        r.iter().map(|x| if *x.iter().min().unwrap()>0{1}else{0}).sum::<i32>()
    }
}
// @lc code=end
