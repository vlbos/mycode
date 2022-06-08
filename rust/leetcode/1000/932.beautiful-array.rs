/*
 * @lc app=leetcode id=932 lang=rust
 *
 * [932] Beautiful Array
 */

// @lc code=start
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
         use std::collections::HashMap;
        fn f(memo:&mut HashMap<i32,Vec<i32>>,n: i32) -> Vec<i32> {
            if let Some(arr)=memo.get(&n){
                return arr.clone();
            }
            let mut ans = vec![0;n as usize];
            if n ==1{
                ans[0]=1;
            }else{
                let mut t = 0;
                for x in f(memo,(n+1)/2){
                    ans[t]=2*x-1;
                    t+=1;
                }
                for x in f(memo,n/2){
                    ans[t]=2*x;
                    t+=1;
                }
            }
            memo.insert(n,ans.clone());
            ans
        }
        let mut m = HashMap::new();
        f(&mut m,n)
    }
}
// @lc code=end

