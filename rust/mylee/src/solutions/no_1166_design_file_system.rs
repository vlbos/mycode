// 1166\. Design File System
// =========================

// You are asked to design a file system which provides two functions:

// *   **createPath(path, value):** Creates a new path and associates a value to it if possible and returns `True`.
// Returns `False` if the path already exists or its parent path doesn't exist.
// *   **get(path):** Returns the value associated with a path or returns `-1` if the path doesn't exist.

// The format of a path is one or more concatenated strings of the form: `/` followed by one or more lowercase English letters.
// For example, `/leetcode` and `/leetcode/problems` are valid paths while an empty string and `/` are not.

// Implement the two functions.

// Please refer to the examples for clarifications.

// **Example 1:**

// **Input:**
// \["FileSystem","createPath","get"\]
// \[\[\],\["/a",1\],\["/a"\]\]
// **Output:**
// \[null,true,1\]
// **Explanation:**
// FileSystem fileSystem = new FileSystem();

// fileSystem.createPath("/a", 1); // return true
// fileSystem.get("/a"); // return 1

// **Example 2:**

// **Input:**
// \["FileSystem","createPath","createPath","get","createPath","get"\]
// \[\[\],\["/leet",1\],\["/leet/code",2\],\["/leet/code"\],\["/c/d",1\],\["/c"\]\]
// **Output:**
// \[null,true,true,2,false,-1\]
// **Explanation:**
// FileSystem fileSystem = new FileSystem();

// fileSystem.createPath("/leet", 1); // return true
// fileSystem.createPath("/leet/code", 2); // return true
// fileSystem.get("/leet/code"); // return 2
// fileSystem.createPath("/c/d", 1); // return false because the parent path "/c" doesn't exist.
// fileSystem.get("/c"); // return -1 because this path doesn't exist.

// **Constraints:**

// *   The number of calls to the two functions is less than or equal to `10^4` in total.
// *   `2 <= path.length <= 100`
// *   `1 <= value <= 10^9`

// **NOTE:** create method has been changed on August 29, 2019 to createPath. Please reset to default code definition to get new method signature.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb)

#[allow(dead_code)]
use std::collections::HashMap;
pub struct FileSystem {
    p2v: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSystem {
    pub fn new() -> Self {
        Self {
            p2v: HashMap::new(),
        }
    }

    pub fn create_path(&mut self, path: String, value: i32) -> bool {
        if self.p2v.contains_key(&path) {
            return false;
        }
        for (i, _) in path.match_indices("/") {
            if i == 0 {
                continue;
            }
            if !self.p2v.contains_key(&path[..i].to_string()) {
                return false;
            }
        }
        self.p2v.insert(path.clone(), value);

        true
    }

    pub fn get(&self, path: String) -> i32 {
        *self.p2v.get(&path).unwrap_or(&-1)
    }
}

/**
 * Your FileSystem object will be instantiated and called as such:
 * let obj = FileSystem::new();
 * let ret_1: bool = obj.create_path(path, value);
 * let ret_2: i32 = obj.get(path);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_create_path_1() {
        let mut fs = FileSystem::new();
        assert!(fs.create_path(String::from("/a"), 1));
        assert_eq!(1, fs.get(String::from("/a")));
    }

    #[test]
    pub fn test_create_path_2() {
        let mut fs = FileSystem::new();
        let testcases = [
            ("/leet", 1),
            ("/leet/code", 2),
            ("/leet/code", 0),
            ("/c/d", 1),
            ("/c", 0),
        ];
        let expected_result = [(true, 0), (true, 0), (false, 2), (false, 0), (false, -1)];
        for (t, e) in testcases.iter().zip(&expected_result) {
            println!("{:?},{:?}", t, e);
            if t.1 == 0 {
                assert_eq!(e.1, fs.get(t.0.to_string()));
            } else {
                assert_eq!(e.0, fs.create_path(t.0.to_string(), t.1));
            }
        }
    }
}
