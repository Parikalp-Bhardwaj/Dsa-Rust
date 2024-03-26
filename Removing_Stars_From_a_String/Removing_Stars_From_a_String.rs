struct Solution{}


impl Solution{
    fn remove_stars(s: String) -> String{
        let mut ans = String::new();
        let s1:String = s.chars().filter_map(|c|{
            if c == '*'{
                ans.pop();
                None
            }else{
                ans.push(c);
                Some(c)
            }}).collect();
        ans
    }
}


fn main(){
    let s = "leet**cod*e".to_string();
    let sol = Solution::remove_stars(s);
    println!("{:?} ",sol);

}