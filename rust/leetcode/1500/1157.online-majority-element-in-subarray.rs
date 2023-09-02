/*
 * @lc app=leetcode id=1157 lang=rust
 *
 * [1157] Online Majority Element In Subarray
 */

// @lc code=start
struct MajorityChecker {
    node: Vec<Vec<i32>>,
    root: Vec<i32>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
         let nn = 20000 + 100;
        let mut node = vec![vec![0; 3]; nn << 5];
        let mut root = vec![0; nn];
        let n = arr.len() as i32;
        let mut tot = 0;
        fn ins(
            l: i32,
            r: i32,
            pre: i32,
            now: &mut i32,
            num: i32,
            tot: &mut i32,
            node: &mut Vec<Vec<i32>>,
        ) {
            *tot += 1;
            *now = *tot;
            let (now, pre) = (*now as usize, pre as usize);
            node[now] = node[pre].clone();
            node[now][2] += 1;
            if l == r {
                return;
            }
            let mid = (l + r) / 2;
            if num <= mid {
                let mut v =node[now][0];
                ins(l, mid, node[pre][0], &mut v, num, tot, node);
                node[now][0]=v;
            } else {
                 let mut v =node[now][1];
                ins(mid + 1, r, node[pre][1],&mut  v, num, tot, node);
                 node[now][1]=v;
            }
        }
        for (i, &v) in arr.iter().enumerate() {
            ins(1, n, root[i], &mut root[i + 1], v, &mut tot, &mut node);
        }
        Self { node, root, n }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        fn search(
            l: i32,
            r: i32,
            start: i32,
            end: i32,
            num: i32,
            node: &Vec<Vec<i32>>,
        ) -> i32 {
            if l == r {
                return l;
            }
            let mid = (l + r) / 2;
            let (start, end) = (start as usize, end as usize);
            if node[node[end][0] as usize][2] - node[node[start][0] as usize][2] >= num {
                return search(l, mid, node[start][0], node[end][0], num, node);
            }
            if node[node[end][1] as usize][2] - node[node[start][1] as usize][2] >= num {
                return search(mid + 1, r, node[start][1], node[end][1], num, node);
            }
            -1
        }
        search(
            1,
            self.n,
            self.root[left as usize],
            self.root[right as usize + 1],
            threshold,
            &self.node,
        )
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */
// @lc code=end



use std::collections::{HashMap};
struct MajorityChecker {
t:HashMap<i32,Vec<i32>>,
a:HashMap<i32,i32>,
s:HashMap<i32,Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {

    fn new(arr: Vec<i32>) -> Self {
        let mut t=HashMap::new();
        let a:HashMap<i32,i32>=arr.iter().enumerate().map(|(i,&v)|(i as i32,v)).collect();
        fn build(i:i32,l:i32,r:i32,a:&HashMap<i32,i32>,t:&mut HashMap<i32,Vec<i32>>){
            if l==r{
                t.insert(i,vec![*a.get(&l).unwrap_or(&0),1]);
                return 
            }
            let mid=(l+r)/2;
            build(i*2,l,mid,a,t);
            build(i*2+1,mid+1,r,a,t);
            let v=if let (Some(a),Some(b))=(t.get(&(i*2)),t.get(&(i*2+1))){
if a[0]==b[0]{
                    vec![a[0],a[1]+b[1]]
                }else if a[1]<b[1]{
                         vec![b[0],b[1]-a[1]]
                }else{
                     vec![a[0],a[1]-b[1]]
                }
            }else{vec![0;2]};
            t.insert(i,v);
        }
        build(1,0,arr.len() as i32-1,&a,&mut t);
        let mut s=HashMap::new();
        for (i,&v) in arr.iter().enumerate(){
            s.entry(v).or_insert(Vec::new()).push(i as i32);
        }
        Self{t,a,s}
    }
    
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
          fn ask(i:i32,l:i32,r:i32,ll:i32,rr:i32,t:&HashMap<i32,Vec<i32>>)->Vec<i32>{
            if l==ll && rr==r{
                return t[&i].clone()
            }
            let mid=(l+r)/2;
            if rr<=mid{
                ask(i*2,l,mid,ll,rr,t)
            }else if ll>mid{
 ask(i*2+1,mid+1,r,ll,rr,t)
            }else{
               let (a,b) = (ask(i*2,l,mid,ll,mid,t), ask(i*2+1,mid+1,r,mid+1,rr,t));
                if a[0]==b[0]{
                    vec![a[0],a[1]+b[1]]
                }else if a[1]<b[1]{
                         vec![b[0],b[1]-a[1]]
                }else{
                     vec![a[0],a[1]-b[1]]
                }
            }
            
        }
        let mut ans=ask(1,0,self.a.len() as i32-1,left,right,&self.t)[0];
        if (self.s[&ans].partition_point(|&v| v<=right) as i32)-(self.s[&ans].partition_point(|&v| v<left) as i32)<threshold{
           -1
        }else{
            ans
        }
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */