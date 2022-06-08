/*
 * @lc app=leetcode id=385 lang=rust
 *
 * [385] Mini Parser
 */

// @lc code=start
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        fn des(sv: &[char], idx: &mut usize) -> Vec<NestedInteger> {
            let mut ans = 0;
            let mut list = Vec::new();
            let mut flag = false;
            let mut sign = false;
            let mut i = *idx;
            while i < sv.len() {
                let c = sv[i];
                if c == '-' {
                    sign = true;
                } else if c.is_ascii_digit() {
                    flag = true;
                    ans *= 10;
                    ans += (c as u8 - b'0') as i32;
                } else if c == '[' {
                    i += 1;
                    list.push(NestedInteger::List(des(sv, &mut i)));
                }
                if i == sv.len() - 1 || c == ']' || c == ',' {
                    if flag {
                        if sign {
                            ans = -ans;
                        }
                        list.push(NestedInteger::Int(ans));
                    }
                    ans = 0;
                    flag = false;
                    sign = false;
                    if c == ']' {
                        break;
                    }
                }
                i += 1;
            }
            *idx = i;
            list
        }
        let sv = s.chars().collect::<Vec<char>>();
        let mut idx = 1;
        if sv[0] == '[' {
            NestedInteger::List(des(&sv, &mut idx))
        } else {
            NestedInteger::Int(s.parse::<i32>().unwrap())
        }
    }
}
// @lc code=end
