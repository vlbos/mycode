/*
 * @lc app=leetcode.cn id=922 lang=rust
 *
 * [922] 按奇偶排序数组 II
 */

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut n =nums;
        let mut i = 0;
        let mut j = 1;
        while i<n.len()-1&&j<n.len(){
            while i<n.len()-1 && n[i]%2==0{
            i+=2;
            }
            if i>=n.len()-1{
            break;
            }
            while j<n.len() && n[j]%2!=0{
                j+=2;
            }
            if  j>=n.len(){
            break;
            }
            let t = n[i];
            n[i]=n[j];
            n[j]=t;
            i+=2;
            j+=2;
        }
        n
    }
}
// @lc code=end

