/*
 * @lc app=leetcode id=1122 lang=rust
 *
 * [1122] Relative Sort Array
 */

// @lc code=start
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut a = Vec::new();
        let mut r = vec![Vec::new();arr2.len()];
        for n in &arr1{
            if let Some(i) = arr2.iter().position(|&x|x==*n){
                r[i].push(*n);
            }else{
                a.push(*n);
            }
        }
        a.sort();
        let mut aa = Vec::new();
        for n in &r{
            aa.extend((*n).clone());
        }
        aa.extend(a);
        aa
    }
}
// @lc code=end

