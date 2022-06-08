/*
 * @lc app=leetcode id=1948 lang=rust
 *
 * [1948] Delete Duplicate Folders in System
 */

// @lc code=start
use std::collections::HashMap;
#[derive(Default)]
struct Trie {
    serial: String,
    children: HashMap<String, Option<Box<Trie>>>,
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = Trie::default();
        for path in &paths {
            let mut cur = &mut root;
            for node in path {
                cur = cur.children.entry(node.clone()).or_insert(Some(Box::new((Trie::default())))).as_mut().unwrap();
            }
        }
        let mut freq = HashMap::new();
        fn construct(node: &mut Trie, freq: &mut HashMap<String, i32>) {
            if node.children.is_empty() {
                return;
            }
            let mut v = Vec::new();
            for (folder, child) in &mut node.children {
                construct(child.as_mut().unwrap(), freq);
                v.push(format!(
                    "{}({})",
                    folder.clone(),
                    child.as_ref().unwrap().serial.clone()
                ));
            }
            v.sort();
            node.serial.push_str(v.concat().as_str());
            *freq.entry(node.serial.clone()).or_insert(0) += 1;
        }
        construct(&mut root,&mut freq);
        let mut ans = Vec::new();
        let mut path = Vec::new();
        fn operate(
            node: &Trie,
            freq: &HashMap<String, i32>,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if *freq.get(&node.serial).unwrap_or(&0) > 1 {
                return;
            }
            if !path.is_empty() {
                ans.push(path.clone());
            }
            for (folder, child) in &node.children {
                path.push(folder.clone());
                operate(child.as_ref().unwrap(), freq, path, ans);
                path.pop();
            }
        }
        operate(&root, &freq, &mut path, &mut ans);
        ans
    }
}
// @lc code=end
