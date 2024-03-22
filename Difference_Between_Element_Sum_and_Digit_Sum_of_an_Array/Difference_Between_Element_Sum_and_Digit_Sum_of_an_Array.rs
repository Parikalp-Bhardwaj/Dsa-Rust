struct Solution{}


impl Solution{
    fn difference_of_sum(nums:Vec<i32>) -> i32{
        let sum1:i32 = nums.iter().sum();
        let mut sum = 0;
        for mut i in nums{
            while i > 0{
                sum += i%10;
                i /= 10;
            }
        }
        i32::abs(sum1 - sum)
        
    }
}

fn main(){
    let nums = vec![1,15,6,3];
    let sol = Solution::difference_of_sum(nums);
    println!("{:?} ",sol);
}