struct Solution{}



impl Solution{
    fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32{

        (0..nums.len()).filter(|i| i.count_ones() == k as u32).map(|i| nums[i]).sum()
        
    }

}


fn main(){

    let nums = vec![5,10,1,5,2];
    let k = 1;
    let sol = Solution::sum_indices_with_k_set_bits(nums,k);
    println!("{:?} ",sol);
}