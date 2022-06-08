/*
 * @lc app=leetcode id=2076 lang=rust
 *
 * [2076] Process Restricted Friend Requests
 */

// @lc code=start
impl Solution {
    pub fn friend_requests(
        n: i32,
        restrictions: Vec<Vec<i32>>,
        requests: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut parent: Vec<i32> = (0..n).collect();
        let mut size = vec![1; n as usize];
        let mut set_count = n;
        fn find_set(x: i32, parent: &mut Vec<i32>) -> i32 {
            if parent[x as usize] == x {
                return x;
            }
            parent[x as usize] = find_set(parent[x as usize], parent);
            parent[x as usize]
        }
        let unite =
            |x: i32, y: i32, set_count: &mut i32, parent: &mut Vec<i32>, size: &mut Vec<i32>| {
                let (x, y) = (find_set(x, parent), find_set(y, parent));
                if x == y {
                    return false;
                }
                let (x, y) = if size[x as usize] < size[y as usize] {
                    (y, x)
                } else {
                    (x, y)
                };
                parent[y as usize] = x;
                size[x as usize] += size[y as usize];
                *set_count -= 1;
                true
            };
        let mut ans = Vec::new();
        for request in &requests {
            let (x, y) = (
                find_set(request[0], &mut parent),
                find_set(request[1], &mut parent),
            );
            if x == y {
                ans.push(true);
                continue;
            }
            let mut check = true;
            for res in &restrictions {
                let (u, v) = (find_set(res[0], &mut parent), find_set(res[1], &mut parent));
                if u == x && v == y || v == x && u == y {
                    check = false;
                    break;
                }
            }
            if check {
                ans.push(true);
                unite(x, y, &mut set_count, &mut parent, &mut size);
            } else {
                ans.push(false);
            }
        }
        ans
    }
}
// @lc code=end
