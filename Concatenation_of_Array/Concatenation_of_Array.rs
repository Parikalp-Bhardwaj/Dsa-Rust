struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        // let n1 = nums.clone();
        // let num = [n1, (nums).to_vec()].concat();
        // num
        let mut res: Vec<i32> = vec![0; nums.len() * 2];
        nums.iter().enumerate().for_each(|(idx, &x)| {
            res[idx] = x;
            res[idx + nums.len()] = x;
        });
        res
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 1];
    println!("{:?} ", Solution::get_concatenation(nums));
}
