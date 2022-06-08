/*
 * @lc app=leetcode id=1089 lang=rust
 *
 * [1089] Duplicate Zeros
 */

// @lc code=start
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n =  arr.len();
        let mut index = Vec::new();
        for (i,n) in arr.iter().enumerate(){
            if  *n==0{
                index.push(i);
            }
        }
        if !index.is_empty(){
            for i in  (0..index.len()).rev(){
                 let r = if i==index.len()-1{n-1}else{index[i+1]+1};
                 for j in (index[i]..r).rev(){
                        if j+i+1<arr.len(){
                            arr[j+i+1]=arr[j];
                        }   
                 }
                if  index[i]+1<arr.len(){
                            arr[index[i]+1]=arr[index[i]];
                }   
            }

        }
    }
}
// @lc code=end

