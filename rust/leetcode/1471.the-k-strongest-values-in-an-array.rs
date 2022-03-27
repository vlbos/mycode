/*
 * @lc app=leetcode id=1471 lang=rust
 *
 * [1471] The k Strongest Values in an Array
 */

// @lc code=start
impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr=arr;
        arr.sort();
        let m = arr[(arr.len()-1)/2];
        arr.sort_by(|a,b| if (a-m).abs()== (b-m).abs(){b.cmp(&a)}else{(b-m).abs().cmp(&(a-m).abs())} );
        arr[..k as usize].to_vec()
    }
}
// @lc code=end

