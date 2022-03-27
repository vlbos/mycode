/*
 * @lc app=leetcode id=1033 lang=rust
 *
 * [1033] Moving Stones Until Consecutive
 */

// @lc code=start
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
       let mut v = [a,b,c];
        v.sort();
        if v[2]-v[1]==1 && v[1]-v[0]==1 {
            return vec![0,0];
        }
        if v[2]-v[1]==1 || v[1]-v[0]==1 ||v[2]-v[1]==2 || v[1]-v[0]==2 {
            return vec![1,v[2]-v[0]-2];
        }
        vec![2,v[2]-v[0]-2]
    }
}
// @lc code=end

