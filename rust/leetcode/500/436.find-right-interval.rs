/*
 * @lc app=leetcode id=436 lang=rust
 *
 * [436] Find Right Interval  [[3,4],[2,3],[1,2]]
 */

// @lc code=start
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = intervals.iter().enumerate().map(|(i,v)| (v[0],i)).collect::<Vec<(i32,usize)>>();
        m.sort_by(|a,b|a.0.cmp(&b.0));
        let mut ans =Vec::new();
        for i  in &intervals{
            let mut index = -1;
            let mut  l = 0;
            let mut  r = m.len()-1;
            let mut pre =m.len();
            while l<=r{
                let mut mid=(r+l)/2;
                if mid<m.len() && m[mid].0>=i[1]{
                    r=mid;
                    index=m[mid].1 as i32;
                }else{
                    l=mid+1;
                }
                if pre==mid{
                break;
                }
                pre=mid;
            }
            ans.push(index);
        }
        ans
    }
}
// @lc code=end

