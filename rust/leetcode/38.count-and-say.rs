/*
 * @lc app=leetcode id=38 lang=rust
 *
 * [38] Count and Say
 */

// @lc code=start
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut ans = String::new();
        for i in 1..=n{
            if i==1{
                ans = "1".to_string();
            }else{
                let a = ans.clone();
                ans = String::new();
                let mut cnt = 0;
                let mut pre = ' ';
                for c in a.chars(){
                    if c!=pre {
                        if cnt>0{
                            ans.push_str(&cnt.to_string());
                            ans.push(pre);
                        }
                        cnt=1;
                        pre=c;
                    }else{
                        cnt+=1;
                    }
                }
                if cnt>0{
                            ans.push_str(&cnt.to_string());
                            ans.push(pre);
                }
            }
        }
        ans
    }
}
// @lc code=end

