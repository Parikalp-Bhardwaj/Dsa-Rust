// fn triangular_sum(nums:&mut Vec<i32>) -> i32{
//     let mut n = nums.len();
//     while n > 1{
//         let mut v1:Vec<i32> = Vec::new();
//         for i in 0..n-1{
//             v1.push((nums[i] + nums[i+1])%10);
//         }
//         n-=1;
//         *nums=v1;
//     }
//     return nums[0];
// }


fn triangular_sum(nums:Vec<i32>) -> i32{
    let mut n = nums.len();
    let mut cl = nums.clone();
    while n > 1{
        let mut v1:Vec<i32> = Vec::new();
        for i in 0..n-1{
            v1.push((cl[i] + cl[i+1])%10);
        }
        n-=1;
        cl=v1;
    }
    return cl[0];
}



fn main(){
    // let mut vec:Vec<i32> = vec![1,2,3,4,5];
    // println!("{} ",triangular_sum(&mut vec))

    let vec:Vec<i32> = vec![1,2,3,4,5];
    println!("{} ",triangular_sum(vec))
}