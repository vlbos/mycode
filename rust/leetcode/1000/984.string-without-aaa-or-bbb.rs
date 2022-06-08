/*
 * @lc app=leetcode id=984 lang=rust
 *
 * [984] String Without AAA or BBB
 */

// @lc code=start
impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let mut a = a;
        let mut b = b;
        let mut wa = true;
        let mut ans = Vec::new();
        while a>0||b>0{
            let n = ans.len();
            if n>=2 && ans[n-1]==ans[n-2]{
                    wa=ans[n-1]=='b';
            }else{
                wa=a>=b;
            }
            if wa{
                a-=1;
                ans.push('a');
            }else{ b-=1;
                ans.push('b');}
        }
        ans.iter().collect()
    }
}
// @lc code=end

