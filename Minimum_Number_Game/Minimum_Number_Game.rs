struct Solution{}

impl Solution{
    fn numberGame(nums: Vec<i32>) -> Vec<i32>{
        let mut num = nums.clone();
        num.sort();
        let mut out:Vec<i32> = Vec::new();
        let mut res:Vec<i32> = Vec::new();
        let mut alice:Vec<i32> = Vec::new();
        let mut bob:Vec<i32> = Vec::new();
        for (i,n) in num.iter().enumerate(){
            if i % 2 == 0{
                alice.push(*n);
            }else{
                bob.push(*n);
            }
        }
        for (a,b) in alice.iter().zip(bob.iter()){
            out.push(*b);
            out.push(*a);
            
        }
        out
    }
}

fn main(){
    let nums:Vec<i32> = vec![5,4,2,3];
    let sol = Solution::numberGame(nums);
    println!("{:?} ",sol);
}