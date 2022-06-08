/*
 * @lc app=leetcode id=1233 lang=rust
 *
 * [1233] Remove Sub-Folders from the Filesystem
 */

// @lc code=start
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
       let mut folder=folder;
        folder.sort();
        let mut i =0;
        let mut ans=Vec::new();
        while i<folder.len(){
            ans.push(folder[i].clone());
            let cnt = folder[i].matches('/').count();
            let mut j = i+1;
            while j<folder.len() && folder[j].starts_with(&folder[i])&& folder[j].matches('/').count()>cnt{
            j+=1;
            }
            i=j;
        }
        ans
    }
}
// @lc code=end

