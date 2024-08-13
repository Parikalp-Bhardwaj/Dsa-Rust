// use std::num::abs;
struct Solution{}


impl Solution{
    pub fn score_of_string(s: String) -> i32 {
        let s1:Vec<char> = s.chars().collect();
        let mut res = 0;
        for idx in 1..s1.len(){
            let a = s1[idx-1].to_ascii_lowercase() as u8;
            let b = s1[idx].to_ascii_lowercase() as u8;
            res += ( a as i32 - b as i32).abs();   
        }
        // res

        let res = s.chars()
                .map(|c|c.to_ascii_lowercase() as i32)
                .collect::<Vec<_>>()
                .windows(2)
                .map(|window|(window[0] - window[1]).abs())
                .sum::<i32>();

        return res
    }

}

fn main(){
    let s = "hello".to_string();
    let sol = Solution::score_of_string(s);
    println!("{:?} ",sol);

}