/*
 * @lc app=leetcode id=912 lang=rust
 *
 * [912] Sort an Array
 */

// @lc code=start
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums=nums;
        
        fn quick_sort(nums:&mut Vec<i32>,l:i32,r:i32){
             let partition=|nums:&mut Vec<i32>,l:i32,r:i32|->i32{
                 let p = nums[r as usize];
                 let mut i= l -1;
                 for j in l..r{
                       if nums[j as usize]<=p{
                           i+=1;
                           nums.swap(i as usize,j  as usize);
                       }
                 }
                  nums.swap(i as usize+1,r  as usize);
                  i+1
            };
            let rand_partition=|nums:&mut Vec<i32>,l:i32,r:i32|->i32{
            use rand::{thread_rng, Rng};
            let mut rng = thread_rng();
            // Exclusive range
            let pos = rng.gen_range(l ,r +1) as usize;
            nums.swap(r as usize,pos);
            partition(nums,l,r)
            };
            if l<r{
                let pos = rand_partition(nums,l,r);
                quick_sort(nums,l,pos-1);
                quick_sort(nums,pos+1,r);
            }
        }
        fn heap_sort(nums:&mut Vec<i32>){
            let max_heapify=|nums:&mut Vec<i32>,i:usize,len:usize|{
                let mut i = i;
                 while (i<<1)+1<=len{
                      let lson = (i<<1)+1;
                      let rson = (i<<1)+2;
                      let mut large= i;
                      if lson<=len && nums[lson]>nums[large]{
                           large=lson;
                      }
                      if rson<=len && nums[rson]>nums[large]{
                           large=rson;
                      }
                      if large!=i{
                              nums.swap(i,large);
                              i=large;  
                      }else{break;}
                 }
                 
            };
            let build_max_heap=|nums:&mut Vec<i32>,len:usize|{
                for i in (0..=len/2).rev(){
                    max_heapify(nums,i,len);
                }
            };
            let mut len = nums.len()-1;
             build_max_heap(nums,len);
             for i in (1..=len).rev(){
                 nums.swap(i,0);
                 len-=1;
                max_heapify(nums,0,len);
             }
            
        }
        fn merge_sort(nums:&mut Vec<i32>,tmp:&mut Vec<i32>,l:usize,r:usize){
            if l>=r{
                return;
            }
            let mid = (l+r)>>1;
            merge_sort(nums,tmp,l,mid);
            merge_sort(nums,tmp,mid+1,r);
            let mut cnt = 0;
            let (mut i,mut j)=(l,mid+1);
            while i<=mid && j<=r{
                if nums[i]<=nums[j]{
                    tmp[cnt]=nums[i];
                    i+=1;
                }else{
                    tmp[cnt]=nums[j];
                    j+=1;
                }
                cnt+=1;
            }
            while i<=mid {
                    tmp[cnt]=nums[i];
                    i+=1;
                    cnt+=1;
            }
            while  j<=r{
                    tmp[cnt]=nums[j];
                    j+=1;
                    cnt+=1;
            }
            for i in 0..cnt{
                nums[l+i]=tmp[i];
            }
        }
        let n = nums.len();
        // quick_sort(&mut nums,0,n as i32-1);
        // heap_sort(&mut nums);
        let mut tmp = vec![0;n];
        merge_sort(&mut nums,&mut tmp,0,n-1);
        nums
    }
}
// @lc code=end

