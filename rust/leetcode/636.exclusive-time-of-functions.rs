/*
 * @lc app=leetcode id=636 lang=rust
 *
 * [636] Exclusive Time of Functions
 */

// @lc code=start
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans= vec![0;n as usize];
        let mut s =  Vec::new();
        let ll = logs[0].split(":").collect::<Vec<&str>>();
        s.push(ll[0].parse::<usize>().unwrap());
        let mut prev= ll[2].parse::<i32>().unwrap();
        for i in 1..logs.len(){
            let ll = logs[i].split(':').collect::<Vec<&str>>();
            let id = ll[0].parse::<usize>().unwrap();
            let tm = ll[2].parse::<i32>().unwrap();
            if ll[1]=="start"{
                if let Some(last)=s.last(){
                    ans[*last]+=tm-prev;
                }
                prev=tm;
                s.push(id);
            }else{
                if let Some(pid)=s.pop(){
                    ans[pid]+=tm-prev+1;
                }
                prev=tm+1;
            }
        }
        ans
    }
}
// @lc code=end

