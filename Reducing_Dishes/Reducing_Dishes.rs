struct Solution{}


impl Solution{
    fn max_satisfaction(satisfaction:Vec<i32>) -> i32{
        let mut s1 = satisfaction.clone();
        s1.sort();
        let mut sum = 0;
        let mut res = 0;
        for i in (0..s1.len()).rev(){
            if sum+s1[i] > 0{
                sum += s1[i];
                res += sum;
            }
            continue
        }
        res
    }
}


fn main(){
    let satisfaction:Vec<i32> = vec![-1, -8, 0, 5, -9];
    let sol = Solution::max_satisfaction(satisfaction);
    println!("{:?} ",sol);
}