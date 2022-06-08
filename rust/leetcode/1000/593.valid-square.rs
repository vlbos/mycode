/*
 * @lc app=leetcode id=593 lang=rust
 *
 * [593] Valid Square
 */

// @lc code=start
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let dist = |p1: &Vec<i32>, p2: &Vec<i32>|->i32{
        (p2[0]-p1[0])*(p2[0]-p1[0])+(p2[1]-p1[1])*(p2[1]-p1[1])};
        let check=|p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>, p4: &Vec<i32>|-> bool{
            dist(p1,p2)>0 && dist(p1,p3)>0 && dist(p1,p2)==dist(p2,p3) && dist(p2,p3)==dist(p3,p4) && dist(p3,p4)==dist(p4,p1) && dist(p1,p3)==dist(p2,p4)
        };
        check(&p1,&p2,&p3,&p4) ||check(&p1,&p3,&p2,&p4)||check(&p1,&p2,&p4,&p3)
    }
}
// @lc code=end

