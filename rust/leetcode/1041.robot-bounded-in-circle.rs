/*
 * @lc app=leetcode id=1041 lang=rust
 *
 * [1041] Robot Bounded In Circle
 */

// @lc code=start
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let dirs=[0,1,0,-1,0];
        let (mut x, mut y )=(0,0);
        let mut cd = 0;
        for i in instructions.chars(){
            match i{
                'L'=>{
                    cd=(cd+4-1)%4;
                },
                'R'=>{
                    cd=(cd+1)%4;
                },
                _=>{
                x+=dirs[cd];
                y+=dirs[cd+1];
                },
            }
        }
        x==0&&y==0 || cd!=0
    }
}
// @lc code=end

