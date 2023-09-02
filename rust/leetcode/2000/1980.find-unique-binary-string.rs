/*
 * @lc app=leetcode id=1980 lang=rust
 *
 * [1980] Find Unique Binary String
 */

// @lc code=start
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut ns = std::collections::HashSet::new();
        for bin_idx in &nums {
            ns.insert(i32::from_str_radix(bin_idx, 2).unwrap());
        }
        let mut val = 0;
        while ns.contains(&val) {
            val += 1;
        }
       format!("{:0w$b}", val,w=nums.len())
    }
}
// @lc code=end
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        use std::collections::HashSet;
        let nums:HashSet<String>=nums.into_iter().collect();
        fn back_track(idx:usize,s:&mut Vec<char>,nums:&HashSet<String>,ans:&mut String){
            if !nums.contains(&s.iter().collect::<String>()){
                *ans=s.iter().collect();
                return
            }
            for i in idx..s.len(){
                s[i]='1';
                back_track(i+1,s,nums,ans);
                if !ans.is_empty(){
                    return
                }
                s[i]='0';
            }
        }
        let mut ans=String::new();
        back_track(0,&mut vec!['0';nums.len()],&nums,&mut ans);
        ans
    }
}