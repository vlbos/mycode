// 555\. Split Concatenated Strings
// ================================

// Given a list of strings, you could concatenate these strings together into a loop, where for each string you could choose to reverse it or not. 
// Among all the possible loops, you need to find the lexicographically biggest string after cutting the loop, which will make the looped string into a regular one.

// Specifically, to find the lexicographically biggest string, you need to experience two phases:

// 1.  Concatenate all the strings into a loop, where you can reverse some strings or not and connect them in the same order as given.
// 2.  Cut and make one breakpoint in any place of the loop, which will make the looped string into a regular one starting from the character at the cutpoint.

// And your job is to find the lexicographically biggest one among all the possible regular strings.

// **Example:**  

// **Input:** "abc", "xyz"
// **Output:** "zyxcba"
// **Explanation:** You can get the looped string "-abcxyz-", "-abczyx-", "-cbaxyz-", "-cbazyx-",   
// where '-' represents the looped status.   
// The answer string came from the fourth looped one,   
// where you could cut from the middle character 'a' and get "zyxcba".

// **Note:**  

// 1.  The input strings will only contain lowercase letters.
// 2.  The total length of all the strings will not over 1,000.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Alibaba](https://leetcode.ca/tags/#Alibaba)

// @lc code=start
impl Solution {
    pub fn split_looped_string(strs: Vec<String>) -> String {
        // let strs = strs
        //     .into_iter()
        //     .map(|s| {
        //         let rev = s.chars().rev().collect::<String>();
        //         String::max(rev, s)
        //     })
        //     .collect::<Vec<String>>();
        // let size = strs.len();
        // let mut res = String::new();
        // for i in 0..size {
        //     let chunk_2 = &strs[(i + 1)..].join("");
        //     let chunk_3 = &strs[0..i].join("");
        //     for j in 0..strs[i].len() {
        //         let s = &strs[i];
        //         let s_rev = &s.chars().rev().collect::<String>();
        //         {
        //             let chunk_1 = &s[j..];
        //             let chunk_4 = &s[..j];
        //             let joined = vec![chunk_1, chunk_2, chunk_3, chunk_4].join("");
        //             res = String::max(res, joined);
        //         }
        //         {
        //             let chunk_1 = &s_rev[j..];
        //             let chunk_4 = &s_rev[..j];
        //             let joined = vec![chunk_1, chunk_2, chunk_3, chunk_4].join("");
        //             res = String::max(res, joined);
        //         }
        //     }
        // }
        // res
        let strs:Vec<String>=strs.into_iter().map(|s| s.chars().rev().collect::<String>().max(s)).collect();
        let mut ans=String::new();
        for (i,st) in strs.iter().enumerate(){
            let (left,right)=strs.split_at(i);
            let (left,right)=(left.concat(),right[1..].concat());
            for s in [st,&st.chars().rev().collect::<String>()]{
                for j in 0..s.len(){
                    ans=ans.max(s[j..].to_string()+right.as_str()+left.as_str()+&s[..j]);
                }
            }
        }
        ans
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::solutions::util::test_tools::map_to_string;

    #[test]
    fn test_split_looped_string_1() {
        assert_eq!(
            Solution::split_looped_string(map_to_string(&["abc", "xyz"])),
            String::from("zyxcba")
        );
    }
}
