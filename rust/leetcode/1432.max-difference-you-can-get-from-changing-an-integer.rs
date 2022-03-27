/*
 * @lc app=leetcode id=1432 lang=rust
 *
 * [1432] Max Difference You Can Get From Changing an Integer
 */

// @lc code=start
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut min_num= num.to_string();
        let mut max_num=min_num.clone();
        let ns = num.to_string();
        let nb = ns.as_bytes();
        if let Some(i)=ns.bytes().position(|x|x!=b'9'){
                max_num=ns.clone().replace(nb[i] as char,"9");
        }
        if ns.as_bytes()[0]!=b'1'{
            min_num=ns.clone().replace(nb[0] as char,"1");
        }else{
             if let Some(i)=ns.bytes().position(|x|x!=b'0'&&x!=nb[0]){
                  min_num=ns.clone().replace(nb[i] as char,"0");
             }
        }
        max_num.parse::<i32>().unwrap()-min_num.parse::<i32>().unwrap()
    }
}
// @lc code=end

