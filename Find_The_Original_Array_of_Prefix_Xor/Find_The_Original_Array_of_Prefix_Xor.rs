fn main(){
    let v = vec![5,2,0,3,1];
    println!("{:?} ",find_array(v));
}

pub fn find_array(pref:Vec<i32>) ->Vec<i32>{
    let mut v1= vec![];
    v1.push(pref[0]);
    for i in 1..pref.len(){
        v1.push(pref[i] ^ pref[i-1]);
    }
    return v1
}


