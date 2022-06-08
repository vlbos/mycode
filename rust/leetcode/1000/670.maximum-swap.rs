/*
 * @lc app=leetcode id=670 lang=rust
 *
 * [670] Maximum Swap
 */

// @lc code=start
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut nb= num.to_string().chars().collect::<Vec<char>>();
        let mut last=vec![0;10];
        for (i,v) in nb.iter().enumerate(){
            last[((*v) as u8-b'0') as usize]=i;
        }
        for (i,v) in nb.iter().enumerate(){
            let j = ((*v) as u8-b'0') as usize;
            for k in (j+1..=9).rev(){
                if last[k]>i{
                    let t = nb[last[k]];
                    nb[last[k]]=nb[i];
                    nb[i]=t;
                    return nb.iter().collect::<String>().parse::<i32>().unwrap();
                }
            }
        }
        num
    }
}
// @lc code=end

