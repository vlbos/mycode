/*
 * @lc app=leetcode id=475 lang=rust
 *
 * [475] Heaters
 */

// @lc code=start
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut heaters = heaters;
        heaters.push(i32::MIN);
        heaters.push(i32::MAX);
        heaters.sort();
        let mut ans = 0;
        for  h in &houses{
                let mut  l = 0;
                let mut  r= heaters.len()-1;
                while l<r{
                    let mid = l + (r-l)/2;
                    if heaters[mid]>*h{
                        r=mid;
                    }else {
                        l=mid+1;
                    }
                }
                let hh =(*h) as i64;
                let d1 = heaters[l as usize] as i64-hh;
                let d2 = hh-heaters[l as usize-1] as i64;
                let d = d1.min(d2) as i32;
                ans = ans.max(d);
        }
        ans
    }
}
// @lc code=end

