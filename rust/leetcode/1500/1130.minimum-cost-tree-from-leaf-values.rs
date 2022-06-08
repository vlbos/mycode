/*
 * @lc app=leetcode id=1130 lang=rust
 *
 * [1130] Minimum Cost Tree From Leaf Values
 */

// @lc code=start
impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut s = Vec::new();
        let mut ans = 0;
        let mut arr=arr;
        for i in 0..n{
            while !s.is_empty() && (*s.last().unwrap()<=arr[i]||i==n-1){
                let b = s.pop().unwrap();
                ans+=
                if !s.is_empty(){
                    arr[i].min(*s.last().unwrap())*b
                }else{
                    arr[i]*b
                };
                arr[i]=arr[i].max(b);
            }
            s.push(arr[i]);
        }
        ans
    }
}
// @lc code=end

