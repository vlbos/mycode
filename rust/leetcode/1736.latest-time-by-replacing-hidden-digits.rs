/*
 * @lc app=leetcode id=1736 lang=rust
 *
 * [1736] Latest Time by Replacing Hidden Digits
 */

// @lc code=start
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut t = time.chars().collect::<Vec<char>>();
        if t[0]=='?' && t[1]=='?'{
            t[0]='2';
            t[1]='3';
        }else if t[0]=='?' {
            if t[1]<'4'{
                 t[0]='2';
            }else{
                t[0]='1';
            }
        }else if  t[1]=='?'{
            if t[0]=='2'{
                 t[1]='3';
            }else{
                t[1]='9';
            }
        }
        if t[3]=='?' && t[4]=='?'{
            t[3]='5';
            t[4]='9';
        }else if t[3]=='?' {
                 t[3]='5';
        }else if  t[4]=='?'{
                t[4]='9';
        }

        t.iter().collect()
    }
}
// @lc code=end

