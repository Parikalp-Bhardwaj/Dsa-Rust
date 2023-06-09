use std::collections::HashMap;
fn main(){
    // let s  = "leetcode";
    // let t = "coats";

    println!("{}",minSteps(s.to_string(),t.to_string()));
    
}

fn minSteps(s:String,t:String) -> i32{
    let mut h1:HashMap<String,i32> = HashMap::new();
   
    for i in s.chars(){
        *h1.entry(i.to_string()).or_insert(0) += 1;
    }
    
    for i in t.chars(){
        *h1.entry(i.to_string()).or_insert(0) -= 1;
    }
    let mut res:i32 = 0;
    let value = -42i32;
    for (_,j) in h1{
        res += j.abs();
    }
    return res
}
