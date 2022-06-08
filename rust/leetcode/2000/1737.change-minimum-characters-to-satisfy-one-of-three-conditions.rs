/*
 * @lc app=leetcode id=1737 lang=rust
 *
 * [1737] Change Minimum Characters to Satisfy One of Three Conditions
 */

// @lc code=start
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let (mut ca, mut cb) = (vec![0; 26], vec![0; 26]);
        for c in a.bytes() {
            ca[(c - b'a') as usize] += 1;
        }
        for c in b.bytes() {
            cb[(c - b'a') as usize] += 1;
        }
        let (an, bn) = (a.len() as i32, b.len() as i32);
        let mut ans = vec![i32::MAX; 3];
        let (mut pa, mut pb) = (0, 0);
        for i in 0..26 {
            if i > 0 {
                ans[0] = ans[0].min(pb + an - pa);
                ans[1] = ans[1].min(pa + bn - pb);
            }
            pa += ca[i];
            pb += cb[i];
            ans[2] = ans[2].min(an + bn - ca[i] - cb[i]);
        }
        *ans.iter().min().unwrap()
    }
}
// @lc code=end
