/*
 * @lc app=leetcode id=1640 lang=rust
 *
 * [1640] Check Array Formation Through Concatenation
 */

// @lc code=start
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut m = vec![-1; 101];
        for (i, a) in arr.iter().enumerate() {
            m[*a as usize] = i as i32;
        }
        if arr.len()==1{
            return arr[0]==pieces[0][0];
        }
        for p in &pieces {
            if p.len() > 1 {
                for i in 0..p.len() - 1 {
                    let pi = p[i] as usize;
                    let pi1 = p[i + 1] as usize;
                    if m[pi] == -1 || m[pi1] == -1 || m[pi]+1 != m[pi1] {
                        return false;
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
