use std::collections::HashMap;
use std::convert::TryInto;
use std::cmp::max;
fn main(){
    let s = "ababcbacadefegdehijhklij".to_string();
    println!("{:?} ",partitionLabels(s));
}

fn partitionLabels(s:String) -> Vec<i32>{
    let mut h1:HashMap<String,i32> = HashMap::new();
    for (i,s) in s.chars().enumerate(){
        h1.insert(s.to_string(),i as i32);
    }
    println!("{:?} ",h1);

    let mut v1:Vec<i32> = Vec::new();
    let mut res = 0;
    let mut out = 0;
    for (i,s) in s.chars().enumerate(){
        res = max(h1[&s.to_string()],res);
        if i == (res as i32){
            v1.push((res+1 - out) as i32);
            out = res+1;
        }
    }
    return v1
}