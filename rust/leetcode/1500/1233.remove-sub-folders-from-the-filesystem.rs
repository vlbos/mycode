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

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        #[derive(Default)]
        struct Trie{
            children:HashMap<String,Trie>,
            idx:i32,
        }
        let mut trie=Trie::default();

        for (i,path) in folder.iter().enumerate(){
            let mut node=&mut trie;
            for name in path.split('/'){
                node=node.children.entry(name.to_string()).or_insert(Trie::default());
            }
            node.idx=i as i32+1;
        }
        let mut ans=Vec::new();
        fn dfs(node:&Trie,folder: &Vec<String>,ans:&mut Vec<String>){
            if node.idx!=0{
                ans.push(folder[node.idx as usize-1].clone());
                return
            }
            for child in node.children.values(){
                dfs(child,folder,ans);
            }
        }
        dfs(&trie,&folder,&mut ans);
        ans
    }
}