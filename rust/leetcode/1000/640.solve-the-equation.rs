/*
 * @lc app=leetcode id=640 lang=rust
 *
 * [640] Solve the Equation
 */

// @lc code=start
impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let lr = equation.split('=').collect::<Vec<&str>>();
        let mut ans = vec![vec![0,0];2];
        for (i,s) in lr.iter().enumerate(){
            let mut t = 0;
            let mut signed = 1;
            let mut signed_index= s.len();
            for (j,c) in s.bytes().enumerate(){
                if c==b'+'||c==b'-'{
                    if t>0{
                        ans[i][1]+=t*signed;
                        t=0;
                    }
                    signed_index=j;
                    if c==b'-'{signed=-1;}else{signed=1}
                }else if c==b'x'{
                    if t==0&&(j==0||j-signed_index==1){
                        t=1;
                    }
                    ans[i][0]+=t*signed;
                    t=0;
                }else{
                    t*=10;
                    t+=(c-b'0') as i32;
                    if j==s.len()-1{
                        ans[i][1]+=t*signed;
                    }
                }
            }
        }
        if  ans[0][0]==ans[1][0] {
            if  ans[0][1]==ans[1][1] {
                return "Infinite solutions".to_string();
            }else {
                return "No solution".to_string();
            }
        }else {
            let x = ans[0][0]-ans[1][0];
            let c = ans[1][1]-ans[0][1];
            if x!=0 && c%x==0{
                return format!("x={}",c/x);
            }
        }
        String::new()
    }
}
// @lc code=end

