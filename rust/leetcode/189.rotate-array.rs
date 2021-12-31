/*
 * @lc app=leetcode id=189 lang=rust
 *
 * [189] Rotate Array
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {        
        let k = (k as usize) %nums.len();
        if k==0{
            return ;
        }
        let n = nums.len();
        let swap =|nums: &mut Vec<i32>,i:usize,j:usize|{
            let mut i = i;
            let mut j =j;
            while i<j{
                let t = nums[i];
                nums[i]=nums[j];
                nums[j]=t;
                i+=1;
                j-=1;
            }
        };
        swap(nums,0,n-1);
        swap(nums,0,k-1);
        swap(nums,k,n-1);
    }
}
// @lc code=end

