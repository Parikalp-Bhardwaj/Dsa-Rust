struct Solution{}

impl Solution{
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut out:String = String::new();
        for (idx,num) in nums.iter().enumerate(){
            match num.as_bytes()[idx]{
                b'0' => {out.push('1')}
                b'1' => {out.push('0')}
                _ => {}
            }
            
        }
        out
    }
}

fn main(){
    let nums:Vec<String> = vec!["01".to_string(),"10".to_string()];
    let sol = Solution::find_different_binary_string(nums);
    println!("{:?} ",sol);
}