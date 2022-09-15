// 1500\. Design a File Sharing System
// ===================================

// We will use a file-sharing system to share a very large file which consists of `m` small **chunks** with IDs from `1` to `m`.

// When users join the system, the system should assign **a unique** ID to them. The unique ID should be used **once** for each user,
// but when a user leaves the system, the ID can be **reused** again.

// Users can request a certain chunk of the file, the system should return a list of IDs of all the users who have this chunk.
// After that, if at least one other has this chunk, the user who requested the chunk **will get it**.

// Implement the `FileSharing` class:

// *   `FileSharing(int m)` Initializes the object with the number of the chunks of the file `m`.
// *   `int join(int[] ownedChunks)`: A new user joined the system owning some chunks of the file,
//  the system should assign an id to the user which is the **smallest positive integer** not taken by any other user. Return the assigned id.
// *   `void leave(int userID)`: The user with `userID` will leave the system, you cannot take file chunks from them anymore.
// *   `int[] request(int userID, int chunkID)`: The user with `userID` requested the file chunk with `chunkID`.
// Return a list of the IDs of all users that own this chunk sorted in ascending order.

// **Follow-ups:**

// *   What happens if the system identifies the user by their IP address instead of their unique ID and users disconnect and connect from the system with the same IP?
// *   If the users in the system join and leave the system frequently without requesting any chunks, will your solution still be efficient?
// *   If all each user join the system one time, request all files and then leave, will your solution still be efficient?
// *   If the system will be used to share `n` files where the `ith` file consists of `m[i]`, what are the changes you have to do?

// **Example:**

// **Input:**
// \["FileSharing","join","join","join","request","request","leave","request","leave","join"\]
// \[\[4\],\[\[1,2\]\],\[\[2,3\]\],\[\[4\]\],\[1,3\],\[2,2\],\[1\],\[2,1\],\[2\],\[\[\]\]\]
// **Output:**
// \[null,1,2,3,\[2\],\[1,2\],null,\[\],null,1\]
// **Explanation:**
// FileSharing fileSharing = new FileSharing(4); // We use the system to share a file of 4 chunks.
// fileSharing.join(\[1, 2\]);    // A user who has chunks \[1,2\] joined the system, assign id = 1 to them and return 1.
// fileSharing.join(\[2, 3\]);    // A user who has chunks \[2,3\] joined the system, assign id = 2 to them and return 2.
// fileSharing.join(\[4\]);       // A user who has chunk \[4\] joined the system, assign id = 3 to them and return 3.
// fileSharing.request(1, 3);   // The user with id = 1 requested the third file chunk, as only the user with id = 2 has the file, return \[2\] . Notice that user 1 now has chunks \[1,2,3\].
// fileSharing.request(2, 2);   // The user with id = 2 requested the second file chunk, users with ids \[1,2\] have this chunk, thus we return \[1,2\]. We don't care if the user has the file and request it, we just return all the users that can give them the file.
// fileSharing.leave(1);        // The user with id = 1 left the system, all the file chunks with them are no longer available for other users.
// fileSharing.request(2, 1);   // The user with id = 2 requested the first file chunk, no one in the system has this chunk, we return empty list \[\].
// fileSharing.leave(2);        // The user with id = 2 left the system, all the file chunks with them are no longer available for other users.
// fileSharing.join(\[\]);        // A user who doesn't have any chunks joined the system, assign id = 1 to them and return 1. Notice that ids 1 and 2 are free and we can reuse them.

// **Constraints:**

// *   `1 <= m <= 10^5`
// *   `0 <= ownedChunks.length <= min(100, m)`
// *   `1 <= ownedChunks[i] <= m`
// *   Values of `ownedChunks` are unique.
// *   `1 <= chunkID <= m`
// *   `userID` is guaranteed to be a user in the system if you **assign** the IDs **correctly**.
// *   At most `10^4` calls will be made to `join`, `leave` and `request`.
// *   Each call to `leave` will have a matching call for `join`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Unknown](https://leetcode.ca/tags/#Unknown)

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
pub struct FileSharing {
    curr_user_id: i32,
    reused_user_ids: BinaryHeap<Reverse<i32>>,
    u2c: HashMap<i32, HashSet<i32>>,
    c2u: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FileSharing {
    pub fn new(_m: i32) -> Self {
        Self {
            curr_user_id: 0,
            reused_user_ids: BinaryHeap::new(),
            u2c: HashMap::new(),
            c2u: HashMap::new(),
        }
    }

    pub fn join(&mut self, owned_chunks: Vec<i32>) -> i32 {
        let id = if let Some(Reverse(v)) = self.reused_user_ids.pop() {
            v
        } else {
            self.curr_user_id += 1;
            self.curr_user_id
        };
        for &c in &owned_chunks {
            self.u2c.entry(id).or_insert(HashSet::new()).insert(c);
            self.c2u.entry(c).or_insert(HashSet::new()).insert(id);
        }
        id
    }

    pub fn leave(&mut self, user_id: i32) {
        self.reused_user_ids.push(Reverse(user_id));
        for &c in self.u2c.get(&user_id).unwrap_or(&HashSet::new()) {
            self.c2u.entry(c).and_modify(|x| {
                x.remove(&user_id);
            });
        }
        self.u2c.remove(&user_id);
    }

    pub fn request(&mut self, user_id: i32, chunk_id: i32) -> Vec<i32> {
        let ans = self
            .c2u
            .get(&chunk_id)
            .unwrap_or(&HashSet::new())
            .iter()
            .cloned()
            .collect();
        self.u2c
            .entry(user_id)
            .or_insert(HashSet::new())
            .insert(chunk_id);
        self.c2u
            .entry(chunk_id)
            .or_insert(HashSet::new())
            .insert(user_id);
        ans
    }
}

/**
 * Your FileSharing object will be instantiated and called as such:
 * let obj = FileSharing::new(m);
 * let ret_1: i32 = obj.join(ownedChunks);
 * obj.leave(userID);
 * let ret_3: Vec<i32> = obj.request(userID, chunkID);
 */

// use std::cmp::Reverse;
// use std::collections::{BinaryHeap, HashMap, BTreeSet};
// pub struct FileSharing {
//     q: BinaryHeap<Reverse<i32>>,
//     u2c: HashMap<i32, Vec<i32>>,
//     c2u: Vec<BTreeSet<i32>>,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl FileSharing {
//     pub fn new(m: i32) -> Self {
//         Self {
//             q: (1..=m).map(|x|Reverse(x)).collect(),
//             u2c: HashMap::new(),
//             c2u: vec![BTreeSet::new();m as usize+1],
//         }
//     }

//     pub fn join(&mut self, owned_chunks: Vec<i32>) -> i32 {
//         let id = if let Some(Reverse(v)) = self.q.pop() {
//             v
//         } else {
//            0
//         };
//         for &c in &owned_chunks {
//             self.c2u[c as usize].insert(id);
//         }
//         self.u2c.insert(id,owned_chunks);
//         id
//     }

//     pub fn leave(&mut self, user_id: i32) {
//         self.q.push(Reverse(user_id));
//         for &c in self.u2c.get(&user_id).unwrap_or(&Vec::new()) {
//             self.c2u[c as usize].remove(&user_id);
//         }
//     }

//     pub fn request(&mut self, user_id: i32, chunk_id: i32) -> Vec<i32> {
//         let ans: Vec<i32>  = self
//             .c2u[chunk_id as usize].iter().cloned().collect();
//         if !ans.is_empty(){
//             self.u2c
//             .entry(user_id)
//             .or_insert(Vec::new())
//             .push(chunk_id);
//         self.c2u[chunk_id as usize].insert(user_id);
//         }
//         ans
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    // ["FileSharing","join","join","join","join","join","request","request","request","request","request"]
    // [[9],[[9,1,8,2,4,5,7,3,6]],[[7,8,9]],[[7,8,3,9,5,1,2,4,6]],[[2,9]],[[]],[1,4],[3,3],[4,8],[2,7],[2,7]]
    // 输出：
    // [null,1,2,3,4,5,[1,3],[3,1],[1,2,3],[1,2,3],[1,2,3]]
    // 预期结果：
    // [null,1,2,3,4,5,[1,3],[1,3],[1,2,3],[1,2,3],[1,2,3]]

    // ["FileSharing","join","join","join","join","join","request","request","request","request","request","request","request","request","request","request","request","request","request","request","request"]
    // [[40],[[35,20,14,40,3,24,10,7,4,31,12,5,39,27,17,36,2,32,37,1,23,30,15,22]],[[33,35,23,15,8,24,3,34,28,19,36,31]],[[]],[[27,18,37,16,9,31,13,22,4,34,36,10,28,26,38]],[[40,39,35,30,16,7,33,32,18,15,25,23,11,22,36,4,8,2,1,29,17,28,3,10,20,37,38,24]],[4,1],[4,27],[2,28],[2,30],[2,26],[3,29],[3,19],[4,6],[4,6],[4,38],[1,33],[5,29],[4,19],[4,26],[5,16]]
    // 输出：
    // [null,1,2,3,4,5,[1,5],[1,4],[2,4,5],[1,5],[4],[5],[2],[],[4],[4,5],[2,5],[3,5],[2,3],[2,4],[4,5]]
    // 预期结果：
    // [null,1,2,3,4,5,[1,5],[1,4],[2,4,5],[1,5],[4],[5],[2],[],[],[4,5],[2,5],[3,5],[2,3],[2,4],[4,5]]
    #[test]
    pub fn test_file_sharing_1() {
        let mut obj = FileSharing::new(4);
        assert_eq!(1, obj.join(vec![1, 2]));
        assert_eq!(2, obj.join(vec![2, 3]));
        assert_eq!(3, obj.join(vec![4]));
        assert_eq!(vec![2], obj.request(1, 3));
        assert_eq!(vec![1, 2], obj.request(2, 2));
        obj.leave(1);
        assert_eq!(Vec::<i32>::new(), obj.request(2, 1));
        obj.leave(2);
        assert_eq!(1, obj.join(Vec::new()));
    }
}
