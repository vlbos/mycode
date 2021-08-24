/*
 * @lc app=leetcode.cn id=350 lang=rust
 *
 * [350] 两个数组的交集 II
 */

// @lc code=start
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut n1 = nums1;
        let mut n2 = nums2;
        n1.sort();
        n2.sort();
        let mut i = 0;
        let mut j =0;
        let mut r = Vec::<i32>::new();
        while i< n1.len() && j<n2.len(){
             if n1[i]==n2[j]{
                r.push(n1[i]);
                i+=1;
                j+=1;
             } else if n1[i]<n2[j]{
                i+=1;
            } else {
                j+=1;
            }
        }
        r
    }
}
// @lc code=end

