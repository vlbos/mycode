// 1088\. Confusing Number II
// ==========================

// We can rotate digits by 180 degrees to form new digits. When 0, 1, 6, 8, 9 are rotated 180 degrees, 
// they become 0, 1, 9, 8, 6 respectively. When 2, 3, 4, 5 and 7 are rotated 180 degrees, they become invalid.

// A _confusing number_ is a number that when rotated 180 degrees becomes a **different** number with each digit valid.
// (Note that the rotated number can be greater than the original number.)

// Given a positive integer `N`, return the number of confusing numbers between `1` and `N` inclusive.

// **Example 1:**

// **Input:** 20
// **Output:** 6
// **Explanation:** 
// The confusing numbers are \[6,9,10,16,18,19\].
// 6 converts to 9.
// 9 converts to 6.
// 10 converts to 01 which is just 1.
// 16 converts to 91.
// 18 converts to 81.
// 19 converts to 61.

// **Example 2:**

// **Input:** 100
// **Output:** 19
// **Explanation:** 
// The confusing numbers are \[6,9,10,16,18,19,60,61,66,68,80,81,86,89,90,91,98,99,100\].

// **Note:**

// 1.  `1 <= N <= 10^9`

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
pub struct Solution {}
impl Solution {
    pub fn confusing_number_ii(n: i32) -> i32 {
        let mut ans=0;
        let m=std::collections::BTreeMap::from([(0,0),(1,1),(6,9),(8,8),(9,6)]);
        let mut s=vec![0];
        let mut found=false;
        let is_confusing_num=|old_num:i32|{
            let mut num=old_num;
            let mut ans=0;
            while num>0{
                if let Some(&v)=m.get(&(num%10)){
                    ans=ans*10+v;
                }else{
                return false;
                }
                num/=10;
            }
            old_num!=ans
        };
        while !found{
            let mut t =Vec::new();
            for &num in &s{
                for (&k,_) in &m{
                    let cur=num*10+k;
                    if cur>n{
                    found=true;
                    break;
                    }
                    if cur!=0{
                    t.push(cur);
                    }
                    if is_confusing_num(cur){
                    println!("=={},{},{}",cur,cur,cur);
                    ans+=1;
                    }
                }
                if found{
                break;
                }
            }
            s=t;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_confusing_number_ii_1() {
        assert_eq!(6, Solution::confusing_number_ii(20));
    }
    #[test]
    fn test_confusing_number_ii_2() {
        assert_eq!(19, Solution::confusing_number_ii(100));
    }
}
