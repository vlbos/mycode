/*
 * @lc app=leetcode id=1913 lang=rust
 *
 * [1913] Maximum Product Difference Between Two Pairs
 */

// @lc code=start
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        // let mut n = nums;
        // n.sort();
        // n[n.len()-1]*n[n.len()-2]-n[1]*n[0]
        let mut v = vec![0;4];
        for n in &nums{
            if v[0]==0{
                v[0]=*n;
            }else if *n>v[0]{
                if v[1]!=0{
                    if v[3]==0{
                        v[3]=v[1];
                    }else if v[3]>v[1]{
                        v[2]=v[3];
                        v[3]=v[1];
                    }else if v[2]==0||v[2]>v[1]{
                         v[2]=v[1];
                    }
                }
                    v[1]=v[0];
                    v[0]=*n;
            } else if *n>v[1]{
                    if v[3]==0{
                        v[3]=v[1];
                    }else if v[3]>v[1]{
                        v[2]=v[3];
                        v[3]=v[1];
                    }else if v[2]==0||v[2]>v[1]{
                         v[2]=v[1];
                    }
                    v[1]=*n;
            }else{
                    if v[3]==0{
                        v[3]=*n;
                    }else if v[3]>*n{
                        v[2]=v[3];
                        v[3]=*n;
                    }else if v[2]==0||v[2]>*n{
                         v[2]=*n;
                    }
            }
        }
        // println!("{:?}",v);
        v[1]*v[0]-v[v.len()-2]*v[v.len()-1]
    }
}
// @lc code=end

