struct Solution{}


impl Solution{
    fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left > right{
            let mid = left + (right + left) / 2;
            if nums[mid] > nums[mid+1]{
                return nums[mid+1]
            }if nums[mid-1]>nums[mid]{
                return nums[mid]
            }if nums[mid] > nums[0]{
                left = mid+1;
            }else{
                right = mid-1;
            }
        }
        return 0
    }
}


fn main(){
    let nums: Vec<i32> = vec![3,4,5,1,2];
    // let nums: Vec<i32> = vec![4,5,6,7,0,1,2];
    let sol = Solution::find_min(nums);
    println!("{:?} ",sol);
}