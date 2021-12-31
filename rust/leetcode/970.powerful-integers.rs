/*
 * @lc app=leetcode id=970 lang=rust
 *
 * [970] Powerful Integers
 */

// @lc code=start
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut ans = std::collections::HashSet::new();
         for i in 0..20{
            if x.pow(i)>bound{
                break;
            }
                for j  in 0..20{
                     if y.pow(j)>bound{
                        break;
                    }
        
                    let d = x.pow(i)+y.pow(j);
                    if d<=bound{
                       ans.insert(d);
                    }
                            
                }
            }
        ans.iter().cloned().collect::<Vec<i32>>()
    }
}
// @lc code=end

