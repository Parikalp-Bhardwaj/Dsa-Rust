struct Solution{}



impl Solution{
    fn num_of_pairs(nums: Vec<String>, target: String) -> i32{
        let mut count = 0; 
        for i in 0..nums.len(){
            for j in 0..nums.len(){
                let res = nums[i].clone() + &nums[j].clone();
                if i == j{
                    continue
                }else if res == target{
                    count+=1;
                }
            }
        }
        return count



    }
}

fn main(){
    // let nums = vec!["777","7","77","77"]
    // .iter().map(|s|s.to_string()).collect::<Vec<String>>();

    // let target = String::from("7777");

    let nums = vec!["1","1","1"]
    .iter().map(|s|s.to_string()).collect::<Vec<String>>();
    let target = String::from("11");
    let sol = Solution::num_of_pairs(nums,target);
    println!("{:?} ",sol);
}