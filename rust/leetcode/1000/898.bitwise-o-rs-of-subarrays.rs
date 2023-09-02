/*
 * @lc app=leetcode id=898 lang=rust
 *
 * [898] Bitwise ORs of Subarrays
 */

// @lc code=start
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut ans = std::collections::HashSet::new();
        let mut cur = std::collections::HashSet::new();
        for &a in &arr {
            let mut cur2 = std::collections::HashSet::new();
            cur2.insert(a);
            ans.insert(a);
            for &c in &cur {
                cur2.insert(a | c);
                ans.insert(a | c);
            }
            cur = cur2;
        }

        ans.len() as _
    }
}
// @lc code=end
impl Solution {
    pub fn subarray_bitwise_o_rs(mut arr: Vec<i32>) -> i32 {
        let mut set=std::collections::HashSet::new();
        for i in 0..arr.len(){
            set.insert(arr[i]);
            for j in (0..i).rev(){
                if arr[i]|arr[j]==arr[j]{
                    break
                }
                arr[j]|=arr[i];
                set.insert(arr[j]);
            }
        }
        set.len() as _
    }
}