/*
 * @lc app=leetcode id=975 lang=rust
 *
 * [975] Odd Even Jump
 */

// @lc code=start
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let make=|b:&Vec<usize>|->Vec<usize>{
            let mut ans =vec![n;n];
            let mut stack=Vec::new();
            for &i in b{
                while !stack.is_empty() && i>stack[stack.len()-1]{
                    ans[stack.pop().unwrap()]=i;
                }
                stack.push(i);
            }

            ans
        };
        let mut b:Vec<usize>=(0..n).collect();
        b.sort_by_key(|x|arr[*x]);
        let odd_next=make(&b);
        b.sort_by_key(|x|-arr[*x]);
        let even_next=make(&b);  
        let mut odd=vec![false;n];
        let mut even=vec![false;n];
        odd[n-1]=true;
        even[n-1]=true;
        for i in (0..n-1).rev(){
            if odd_next[i]!=n{
                odd[i]=even[odd_next[i]];
            }
            if even_next[i]!=n{
                even[i]=odd[even_next[i]];
            }
        }
        odd.iter().filter(|&x|*x).count() as _
    }
}
// @lc code=end

