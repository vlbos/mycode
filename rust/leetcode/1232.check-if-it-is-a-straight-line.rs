/*
 * @lc app=leetcode id=1232 lang=rust
 *
 * [1232] Check If It Is a Straight Line
 */

// @lc code=start
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len()==2{
        return true;
        }
        let x=coordinates[1][0]-coordinates[0][0];
        let y=coordinates[1][1]-coordinates[0][1];
        for i in 2..coordinates.len(){
            if y*(coordinates[i][0]-coordinates[0][0])!=x*(coordinates[i][1]-coordinates[0][1]){
            return false;
            }
        }
        true
    }
}
// @lc code=end

