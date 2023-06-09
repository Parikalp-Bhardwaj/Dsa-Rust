fn partitionArray(nums:Vec<i32>,k:i32) ->i32{
    let mut j = 0;
    let mut res = 1;
    let mut n1 = nums.clone();
    n1.sort();
    n1.reverse();
    for i in 1..n1.len(){
        if n1[j] - n1[i] > k {
            res+=1;
            j=i;
        }
    }
    return res
}


fn main(){
    let nums:Vec<i32> = vec![3, 6, 1, 2, 5];
    let k:i32 = 2;
    println!("{:?} ",partitionArray(nums,k))
}
