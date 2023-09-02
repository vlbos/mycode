/*
 * @lc app=leetcode id=306 lang=rust
 *
 * [306] Additive Number
 */

// @lc code=start
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for i in 1..num.len() - 1 {
            if i > 1 && &num[0..1] == "0" {
                continue;
            }
            for j in i + 1..num.len() {
                if j - i > 1 && &num[i..i + 1] == "0" {
                    continue;
                }
                let mut n1 = (&num[0..i]).parse::<u64>().unwrap();
                let mut n2 = (&num[i..j]).parse::<u64>().unwrap();
                let mut s = (&num[0..j]).to_string();
                loop {
                    let n3 = n1 + n2;
                    n1 = n2;
                    n2 = n3;
                    s += n3.to_string().as_str();
                    if s == num {
                        return true;
                    }
                    if !num.starts_with(&s) {
                        break;
                    }
                }
            }
        }
        false
    }
}
// @lc code=end

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn dfs(i:usize,num: &String,list:&mut Vec<Vec<u8>>)-> bool{
            let n=num.len();
            if i==n{
                return list.len()>2
            }
            let bn=num.as_bytes();
            let m=if bn[i]==b'0'{i+1}else{n};
            let mut cur=Vec::new();
            for j in i..m{
                cur.insert(0,bn[j]-b'0');
                if list.len()<2||check(&list[list.len()-2],&list[list.len()-1],&cur){
                    list.push(cur.clone());
                    if dfs(j+1,num,list){
                        return true
                    }
                    list.pop();
                }
            }
    
            false
        }
        fn check(a:&Vec<u8>,b:&Vec<u8>,c:&Vec<u8>)->bool{
            let mut carry=0;
            let mut ans=Vec::new();
            for i in 0..a.len().max(b.len()){
                if i<a.len(){
                    carry+=a[i];
                }
                if i<b.len(){
                    carry+=b[i];
                }
                ans.push(carry%10);
                carry/=10;
            }
            if carry>0{
                ans.push(1);
            }
            if ans.len()!=c.len(){
                false
            }else{
                ans.iter().zip(c).all(|(&x,&y)| x==y)
            }
        }
        let mut list=Vec::new();
        dfs(0,&num,&mut list)
    }
}