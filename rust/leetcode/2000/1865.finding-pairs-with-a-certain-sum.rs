/*
 * @lc app=leetcode id=1865 lang=rust
 *
 * [1865] Finding Pairs With a Certain Sum
 */

// @lc code=start
use std::collections::HashMap;
struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    cnt:HashMap<i32,i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut cnt = HashMap::new();
        for &v in &nums2{
            *cnt.entry(v).or_insert(0)+=1;
        }
        Self { nums1, nums2 ,cnt}
    }

    fn add(&mut self, index: i32, val: i32) {
        *self.cnt.entry(self.nums2[index as usize]).or_insert(0) -= 1;
        self.nums2[index as usize]+=val;
        *self.cnt.entry(self.nums2[index as usize]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut ans = 0;
        for &v in &self.nums1{
            let d = tot-v;
            if let Some(c)=self.cnt.get(&d){
                ans+=c;
            }
        }
        ans
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
// @lc code=end
