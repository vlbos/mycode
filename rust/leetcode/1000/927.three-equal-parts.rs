/*
 * @lc app=leetcode id=927 lang=rust
 *
 * [927] Three Equal Parts
 */

// @lc code=start
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let sum = arr.iter().sum::<i32>();
        if sum % 3 > 0 {
            return vec![-1, -1];
        }
        let t = sum / 3;
        if t == 0 {
            return vec![0, arr.len() as i32 - 1];
        }
        let mut breaks = Vec::new();
        let mut ones = 0;
        for (i, &x) in arr.iter().enumerate() {
            if x == 0 {
                continue;
            }
            ones += 1;
            if [1, t + 1, t * 2 + 1].iter().any(|v| *v == ones) {
                breaks.push(i);
            }
            if [t, t * 2, t * 3].iter().any(|v| *v == ones) {
                breaks.push(i);
            }
        }
        let (i1, j1, i2, j2, i3, j3) = (
            breaks[0], breaks[1], breaks[2], breaks[3], breaks[4], breaks[5],
        );
        if arr[i1..j1 + 1] != arr[i2..j2 + 1] || arr[i1..j1 + 1] != arr[i3..j3 + 1] {
            return vec![-1, -1];
        }
        let (x, y, z) = (i2 - j1 - 1, i3 - j2 - 1, arr.len() - j3 - 1);
        if x < z || y < z {
            return vec![-1, -1];
        }
        vec![(j1+z) as i32,(j2+z+1) as i32]
    }
}
// @lc code=end
