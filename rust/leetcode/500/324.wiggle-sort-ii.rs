/*
 * @lc app=leetcode id=324 lang=rust
 *
 * [324] Wiggle Sort II
 */

// @lc code=start
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n2 = nums.len()/2;
        fn quick_select(nums : &mut Vec<i32>,start:usize,end:usize,mid:usize){
            let mut k = nums[end-1];
            let mut i = start;
            let mut j=start;
            while j<end{
                if nums[j]<=k{
                    let t = nums[j];
                    nums[j]=nums[i];
                    nums[i]=t;
                    i+=1;
                    j+=1;
                }else{
                    j+=1;
                }
            }
            if i>mid+1{
                quick_select(nums,start,i-1,mid);
            }else if i<=mid{
                quick_select(nums,i,end,mid);
            }
        }
        quick_select(nums,0,nums.len(),n2);
        let mid = nums[n2];
        let  n = nums.len()|1;
        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len()-1;
        while j<=k{
                let rj = ((2*j)+1)%n;
                if nums[rj]>mid{
                    let ri = ((2*i)+1)%n;
                    let t = nums[rj];
                    nums[rj]=nums[ri];
                    nums[ri]=t;
                    i+=1;
                    j+=1;
                }else if nums[rj]<mid{
                    let rk = ((2*k)+1)%n;
                    let t = nums[rj];
                    nums[rj]=nums[rk];
                    nums[rk]=t;
                    k-=1;
                }else{
                    j+=1;
                }
        }
    }
}
// @lc code=end

