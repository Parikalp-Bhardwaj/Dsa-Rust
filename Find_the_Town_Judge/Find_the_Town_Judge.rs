use std::collections::HashMap;
struct Solution{}

impl Solution{
    fn find_judge(n:i32, trust:Vec<Vec<i32>>) -> i32{
        let mut hash:HashMap<i32, i32> = HashMap::new();
        if trust.is_empty() {
       return if n == 1 { 1 } else { -1 };
       }
       for v1 in trust.iter(){
           let v = v1[0];
           let u = v1[1];
           *hash.entry(v).or_insert(0)-=1;
           *hash.entry(u).or_insert(0)+=1;
       }
       for (i,_) in hash.iter(){
           if let Some(v) = hash.get(&i){
               if *v == (n-1){
                   return *i
               }
           }
       }

       -1
    }
}

fn main(){
    let n = 2;
    // let trust:Vec<Vec<i32>> = vec![vec![1,2]];
    let n = 1;
    // let trust:Vec<Vec<i32>> = vec![vec![1,3],vec![2,3]];
    let trust = vec![];
    let sol = Solution::find_judge(n,trust);
    println!("{:?} ",sol);
}