use std::collections::HashSet;
fn main(){
    let nums:Vec<i32> = vec![1,13,10,12,31];
    println!("{} ",countDistinctIntegers(nums))

}

fn countDistinctIntegers(nums:Vec<i32>) -> i32{
    let mut set:HashSet<i32> = HashSet::new();
    for num in nums.iter(){
        set.insert(*num);
        let r = num.to_string().chars().rev().collect::<String>();
        let my_int: i32 = r.parse().unwrap();
        set.insert(my_int);
    }

    return set.len() as i32;
}