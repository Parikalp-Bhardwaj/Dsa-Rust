use std::collections::HashMap;
struct Solution{}


impl Solution{
    fn advantage_count(nums1:Vec<i32>, nums2:Vec<i32>) -> i32{
        let mut n1 = nums1.clone();
        n1.sort();
        let mut b1:HashMap<i32,usize> = HashMap::new();
        for (idx,&num) in nums2.iter().enumerate() {
            b1.insert(num,idx);
        }
        let mut v1:Vec<(&i32,&usize)> = b1.iter().collect(); 
        v1.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:?} ",v1);
        println!("{:?} n1 ",n1);
        let mut res:Vec<i32> = vec![0;n1.len()];
        println!("{:?} res",res);
        let mut l = 0;
        let mut r = n1.len() -1 ;
        let len = b1.iter().count();

        if len > 0 {
            for i in (0..len - 1).rev() {
                let idx = v1[i].1;
                let val = v1[i].0;
                if n1[r] > *val {
                    res[*idx] = n1[r];
                    r -= 1;
                } else {
                    res[*idx] = n1[l];
                    l += 1;
                }
            }
        }
        println!("{:?} ",res);
        return 0
    }
}

fn main(){
    let nums1 = vec![2,7,11,15];
    let nums2 = vec![1,10,4,11];
    let sol = Solution::advantage_count(nums1,nums2);
    println!("{:?} ",sol);
}