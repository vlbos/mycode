/*
 * @lc app=leetcode.cn id=697 lang=rust
 *
 * [697] 数组的度
 */

// @lc code=start
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        
        let mut m = std::collections::HashMap::<i32,i32>::new();
        for n in &nums{
            let mut v= m.entry(*n).or_insert(0);
            *v+=1;
        }
        if nums.len()==m.len(){
        return 1;
        }
        let mut max=0; 
        let mut maxa=Vec::<i32>::new();
        for (k,v) in m{
            if v>max{
                maxa=vec![k].to_vec();
                max = v;
            }else if v==max{
                maxa.push(k);
            }
        }
        let mut min = i32::MAX;
        let mut start=vec![usize::MAX;maxa.len()];
        let mut end=vec![0;maxa.len()];
        for (i,n) in nums.iter().enumerate(){
            for (j,a) in maxa.iter().enumerate(){
                if *a==*n {
                    if start[j]==usize::MAX{
                        start[j] =i;
                    }
                    else if i>end[j]{
                        end[j] =i;
                    }
                }
            }
        }
        for i in 0..maxa.len(){
               min = min.min((end[i]-start[i]) as i32);
        }
        min+1 as i32
    }
}
// @lc code=end

