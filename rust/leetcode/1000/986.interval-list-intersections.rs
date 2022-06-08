/*
 * @lc app=leetcode id=986 lang=rust
 *
 * [986] Interval List Intersections
 */

// @lc code=start
impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let (mut i,mut j )=(0,0);
        while i<first_list.len() && j<second_list.len(){
            let lo = first_list[i][0].max(second_list[j][0]);
            let hi = first_list[i][1].min(second_list[j][1]);
            if lo<=hi{
            ans.push(vec![lo,hi]);
            }
            if first_list[i][1] < second_list[j][1]{
            i+=1;
            }else{
            j+=1;
            }
        }
        ans
    }
}
// @lc code=end

