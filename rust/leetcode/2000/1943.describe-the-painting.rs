/*
 * @lc app=leetcode id=1943 lang=rust
 *
 * [1943] Describe the Painting
 */

// @lc code=start
impl Solution {
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let mut color = std::collections::HashMap::new();
        for segment in &segments {
            let (l, r, c) = (segment[0], segment[1], segment[2] as i64);
            *color.entry(l).or_insert(0) += c;
            *color.entry(r).or_insert(0) -= c;
        }
        let mut axis = color.iter().map(|(&k,&v)|(k,v)).collect::<Vec<(i32, i64)>>();
        axis.sort();
        for i in 1..axis.len() {
            axis[i].1 += axis[i - 1].1;
        }
        let mut ans = Vec::new();
        for i in 0..axis.len() - 1 {
            if axis[i].1 > 0 {
                ans.push(vec![axis[i].0 as i64, axis[i + 1].0 as i64, axis[i].1]);
            }
        }
        ans
    }
}
// @lc code=end
