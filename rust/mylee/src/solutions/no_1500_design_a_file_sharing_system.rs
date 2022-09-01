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

#[cfg(test)]
mod test {
    use super::*;

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
