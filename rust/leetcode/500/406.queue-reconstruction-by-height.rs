/*
 * @lc app=leetcode id=406 lang=rust
 *
 * [406] Queue Reconstruction by Height
 */

// @lc code=start
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut p = people;
        p.sort_by(|a,b| if b[0]<a[0]||(a[0]==b[0] && a[1]<b[1]) {Ordering::Less}else{Ordering::Greater});
        let mut ans = Vec::new();
        for v in p{
            ans.insert(v[1] as usize,v);
        }
        ans
    }
}
// @lc code=end

