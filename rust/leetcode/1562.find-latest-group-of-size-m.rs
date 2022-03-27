/*
 * @lc app=leetcode id=1562 lang=rust
 *
 * [1562] Find Latest Group of Size M
 */

// @lc code=start
impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let n = arr.len();
        let mut end_points=vec![(-1,-1);n+1];
        let mut cnt = 0;
        let mut ans=-1;
        for (i,&v) in arr.iter().enumerate(){
            let (mut l,mut r)=(v,v);
            if v>1 && end_points[v as usize -1].0!=-1{
                let i1=v as usize -1;
                l =end_points[i1].0;
                if end_points[i1].1-end_points[i1].0+1==m{
                    cnt-=1;
                }
            }
            if v<n as i32 && end_points[v as usize +1].1!=-1{
                let i1=v as usize +1;
                r =end_points[i1].1;
                if end_points[i1].1-end_points[i1].0+1==m{
                    cnt-=1;
                }
            }
            if r-l+1==m{
                cnt+=1;
            }
            if cnt>0{
            ans=i as i32+1;
            }
            end_points[l as usize]=(l,r);
            end_points[r as usize]=(l,r);       
        }
        ans
    }
}
// @lc code=end
