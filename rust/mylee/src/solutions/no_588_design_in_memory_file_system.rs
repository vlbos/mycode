// 588\. Design In-Memory File System
// ==================================

// Design an in-memory file system to simulate the following functions:

// `ls`: Given a path in string format. If it is a file path, return a list that only contains this file's name.
// If it is a directory path, return the list of file and directory names **in this directory**.
// Your output (file and directory names together) should in **lexicographic order**.

// `mkdir`: Given a **directory path** that does not exist, you should make a new directory according to the path.
// If the middle directories in the path don't exist either, you should create them as well. This function has void return type.

// `addContentToFile`: Given a **file path** and **file content** in string format.
//  If the file doesn't exist, you need to create that file containing given content.
// If the file already exists, you need to **append** given content to original content. This function has void return type.

// `readContentFromFile`: Given a **file path**, return its **content** in string format.

// **Example:**

// **Input:**
// \["FileSystem","ls","mkdir","addContentToFile","ls","readContentFromFile"\]
// \[\[\],\["/"\],\["/a/b/c"\],\["/a/b/c/d","hello"\],\["/"\],\["/a/b/c/d"\]\]

// **Output:**
// \[null,\[\],null,null,\["a"\],"hello"\]

// **Explanation:**
// ![filesystem](https://assets.leetcode.com/uploads/2018/10/12/filesystem.png)

// **Note:**

// 1.  You can assume all file or directory paths are absolute paths which begin with `/` and do not end with `/` except that the path is just `"/"`.
// 2.  You can assume that all operations will be passed valid parameters and users will not attempt to retrieve file content or list a directory or file that does not exist.
// 3.  You can assume that all directory names and file names only contain lower-case letters, and same names won't exist in the same directory.

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb) [Amazon](https://leetcode.ca/tags/#Amazon) [Baidu](https://leetcode.ca/tags/#Baidu)

// @lc code=start
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum FSNode {
//     Directory {
//         children: BTreeMap<String, Rc<RefCell<FSNode>>>,
//     },
//     File {
//         address: usize,
//     },
// }

// impl FSNode {
//     pub fn   new_directory() -> Self {
//         FSNode::Directory {
//             children: BTreeMap::new(),
//         }
//     }

//     pub fn   new_file(address: usize) -> Self {
//         FSNode::File { address }
//     }
// }
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub struct FileSystem {
    // vfs: Rc<RefCell<FSNode>>,
    // files: Vec<String>,
    dirs: HashMap<String, BTreeSet<String>>,
    files: HashMap<String, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSystem {
    pub fn   new() -> Self {
        // Self {
        //     vfs: Rc::new(RefCell::new(FSNode::new_directory())),
        //     files: vec![],
        // }
        Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    //pub fn  vcd(&self, ps: &[&str]) -> Option<(String, Rc<RefCell<FSNode>>)> {
    //     let mut curr = self.vfs.clone();
    //     let mut key = String::new();
    //     for p in ps {
    //         curr = {
    //             let mut curr_node = curr.borrow_mut();
    //             match *curr_node {
    //                 FSNode::Directory { ref mut children } => children[*p].clone(),
    //                 _ => {
    //                     return None;
    //                 }
    //             }
    //         };
    //         key = p.to_string();
    //     }
    //     Some((key, curr))
    // }

    pub fn   ls(&self, path: String) -> Vec<String> {
        // let ps = path
        //     .split("/")
        //     .skip(1)
        //     .filter(|s| !s.is_empty())
        //     .collect::<Vec<_>>();
        // let directory_op = self.vcd(&ps[0..ps.len()]);
        // if let Some((key, directory)) = directory_op {
        //     let directory_node = directory.borrow_mut();
        //     match *directory_node {
        //         FSNode::Directory { ref children } => children
        //             .iter()
        //             .map(|(k, _)| k.clone())
        //             .collect::<Vec<String>>(),
        //         _ => vec![key],
        //     }
        // } else {
        //     vec![]
        // }
        if self.files.contains_key(&path) {
            return vec![if let Some(i) = path.rfind("/") {
                path[i + 1..].to_string()
            } else {
                path
            }];
        }
        if let Some(df) = self.dirs.get(&path) {
            df.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn   mkdir(&mut self, path: String) {
        // let ps = path
        //     .split("/")
        //     .skip(1)
        //     .filter(|s| !s.is_empty())
        //     .collect::<Vec<_>>();
        // let mut curr = self.vfs.clone();
        // for p in ps {
        //     curr = {
        //         let mut curr_node = curr.borrow_mut();
        //         match *curr_node {
        //             FSNode::Directory { ref mut children } => {
        //                 if children.contains_key(p) {
        //                     children[p].clone()
        //                 } else {
        //                     let new_node = Rc::new(RefCell::new(FSNode::new_directory()));
        //                     children.insert(String::from(p), new_node.clone());
        //                     new_node
        //                 }
        //             }
        //             _ => unreachable!(),
        //         }
        //     };
        // }
        let mut dir = String::from("/");
        for s in path.split("/") {
            if s.is_empty() {
                continue;
            }
            self.dirs
                .entry(dir.clone())
                .or_insert(BTreeSet::new())
                .insert(s.to_string());
            if dir.len() > 1 {
                dir += "/";
            }
            dir += s;
        }
    }

    pub fn   add_content_to_file(&mut self, file_path: String, content: String) {
        // let ps = file_path
        //     .split("/")
        //     .skip(1)
        //     .filter(|s| !s.is_empty())
        //     .collect::<Vec<_>>();
        // if ps.len() == 0 {
        //     return;
        // }
        // let directory = self.vcd(&ps[0..ps.len() - 1]).unwrap().1;
        // let file_name = ps[ps.len() - 1];
        // let mut directory_node = directory.borrow_mut();
        // match *directory_node {
        //     FSNode::Directory { ref mut children } => {
        //         let file_ref = if children.contains_key(file_name) {
        //             children[file_name].clone()
        //         } else {
        //             let new_address = self.files.len();
        //             self.files.push(String::new());
        //             let new_node = Rc::new(RefCell::new(FSNode::new_file(new_address)));
        //             children.insert(String::from(file_name), new_node.clone());
        //             new_node
        //         };
        //         let file_node = file_ref.borrow();
        //         match *file_node {
        //             FSNode::File { address } => {
        //                 self.files[address] += &content;
        //             }
        //             _ => unreachable!(),
        //         };
        //     }
        //     _ => unreachable!(),
        // }
        let (dir, file) = if let Some(i) = file_path.rfind("/") {
            (file_path[..i].to_string(), file_path[i + 1..].to_string())
        } else {
            (String::from("/"), file_path.clone())
        };
        if !self.dirs.contains_key(&dir) {
            self.mkdir(dir.clone());
        }
        self.dirs
            .entry(dir.clone())
            .or_insert(BTreeSet::new())
            .insert(file);
        self.files
            .entry(file_path)
            .or_insert(String::new())
            .push_str(content.as_str());
    }

   pub fn  read_content_from_file(&self, file_path: String) -> String {
        // let ps = file_path
        //     .split("/")
        //     .skip(1)
        //     .filter(|s| !s.is_empty())
        //     .collect::<Vec<_>>();
        // if ps.len() == 0 {
        //     return String::new();
        // }
        // let directory = self.vcd(&ps[0..ps.len() - 1]).unwrap().1;
        // let file_name = ps[ps.len() - 1];
        // let directory_node = directory.borrow_mut();
        // match *directory_node {
        //     FSNode::Directory { ref children } => {
        //         let file_ref = children[file_name].clone();
        //         let file_node = file_ref.borrow();
        //         match *file_node {
        //             FSNode::File { address } => self.files[address].clone(),
        //             _ => unreachable!(),
        //         }
        //     }
        //     _ => unreachable!(),
        // }
        self.files.get(&file_path).unwrap_or(&String::new()).clone()
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
   pub fn  test_file_system_1() {
        let mut fs = FileSystem::new();
        assert_eq!(fs.ls(String::from("/")), Vec::<String>::new());
        fs.mkdir(String::from("/a/b/c"));
        fs.add_content_to_file(String::from("/a/b/c/d"), String::from("hello"));
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["a"]);
        assert_eq!(
            fs.read_content_from_file(String::from("/a/b/c/d")),
            String::from("hello")
        );
    }

    #[test]
   pub fn  test_file_system_2() {
        let mut fs = FileSystem::new();
        fs.mkdir(String::from("/goowmfn"));
        assert_eq!(fs.ls(String::from("/goowmfn")), Vec::<String>::new());
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["goowmfn"]);
        fs.mkdir(String::from("/z"));
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["goowmfn", "z"]);
        assert_eq!(fs.ls(String::from("/")), lc_vec_s!["goowmfn", "z"]);
        fs.add_content_to_file(String::from("/goowmfn/c"), String::from("shetopcy"));
        assert_eq!(fs.ls(String::from("/z")), Vec::<String>::new());
        assert_eq!(fs.ls(String::from("/goowmfn/c")), lc_vec_s!["c"]);
        assert_eq!(fs.ls(String::from("/goowmfn")), lc_vec_s!["c"]);
    }
}
