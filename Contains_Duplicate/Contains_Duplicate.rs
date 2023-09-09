use std::collections::HashSet;
struct Solution {}

impl Solution {
    fn contain_duplicate(nums: Vec<i32>) -> bool {
        let mut hashset: HashSet<i32> = HashSet::new();
        let len = nums.len();
        for num in nums.into_iter() {
            hashset.insert(num as i32);
        }
        if hashset.len() != len {
            return true;
        }
        return false;
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 1];
    let sol = Solution::contain_duplicate(nums);
    println!("{:?} ", sol);
}
