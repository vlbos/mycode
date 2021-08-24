/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 */

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut r = Vec::<String>::new();
       if nums.is_empty(){
            return r;
        }
        if nums.len()==1{
        return vec![nums[0].to_string()].to_vec();
        }
        let mut s = String::new();
        let mut a = 0;
        let mut b =0;
        for (i,n) in nums.iter().enumerate(){
            if 0==i  {
                a = *n;
                b = *n;
                s.push_str(&n.to_string()); 
            }
            else if *n == b+1 {
                 b=*n;
            } 
            else{
                if a==b{
                    r.push(s);
                }else{
                    s.push_str("->");
                    s.push_str(&b.to_string());
                    r.push(s);
                }
                a = *n;
                b = *n;
                s = String::new();
                s.push_str(&n.to_string()); 
            }
        }
      
                if a==b{
                    r.push(s);
                }else{
                    s.push_str("->");
                    s.push_str(&b.to_string());
                    r.push(s);
                }
        r
    }
}
// @lc code=end

