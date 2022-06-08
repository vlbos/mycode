/*
 * @lc app=leetcode id=1386 lang=rust
 *
 * [1386] Cinema Seat Allocation
 */

// @lc code=start
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut occupied= std::collections::HashMap::new();
        for (i,v) in reserved_seats.iter().enumerate(){
            if v[1]<2||v[1]>9{
                    continue;
            }
            *occupied.entry(v[0]).or_insert(0)|=1<<(v[1]-2);
        }
        let mut ans = 2*(n-occupied.len() as i32);
        for (row,mask) in &occupied{
            if [0b00001111,0b11000011,0b11110000].iter().any(|x|*x|mask==*x){
                ans+=1;
            }
        }
        ans
    }
}
// @lc code=end

