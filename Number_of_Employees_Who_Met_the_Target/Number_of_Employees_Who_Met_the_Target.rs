struct Solution{}


impl Solution{
    fn number_of_employees_who_met_target(hours:Vec<i32>, target:i32) -> i32{
        return hours.iter().filter(|&x|x >= &target).count() as i32;
        
    }
}

fn main(){
    let hours:Vec<i32> = vec![0,1,2,3,4];
    let target = 2;
    let sol = Solution::number_of_employees_who_met_target(hours,target);
    println!("{:?} ",sol);
}