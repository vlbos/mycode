/*
 * @lc app=leetcode id=1311 lang=rust
 *
 * [1311] Get Watched Videos by Your Friends
 */

// @lc code=start
impl Solution {
    pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
        let mut used =vec![false;friends.len()];
        let mut q = std::collections::VecDeque::new();
        q.push_back(id as usize);
        used[id as usize]=true;
        for _ in 0..level{
             let len=q.len();
             for _ in 0..len{
                 if let Some(i)=q.pop_front(){
                        for &f in &friends[i]{
                            let fi = f as usize;
                            if !used[fi]{
                                     q.push_back(fi);
                                     used[fi]=true;
                            }
                        }
                 }
             }
        }
        let mut m = std::collections::HashMap::new();
        while let Some(i)=q.pop_front(){
            for v in &watched_videos[i]{
                *m.entry(v.clone()).or_insert(0)+=1;
            }
        }
        let mut vl :Vec<(i32,String)>= m.iter().map(|(k,v)|(*v,k.clone())).collect();
        vl.sort();
        vl.iter().map(|x|x.1.clone()).collect()
    }
}
// @lc code=end

