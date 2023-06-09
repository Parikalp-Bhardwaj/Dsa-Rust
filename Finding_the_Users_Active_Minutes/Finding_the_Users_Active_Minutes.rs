use std::collections::HashMap;
fn finding_users_active_minutes(logs:Vec<Vec<i32>>,k:i32) ->Vec<i32>{
    let mut res = vec![0;k as usize];
    let mut h1:HashMap<i32,Vec<i32>> = HashMap::new();
    for i in logs{
       let (k,v)= (i[0],i[1]);
        h1.entry(k).or_default().push(v);
        
    }

    for (_,mut i) in h1{
        i.sort();
        i.dedup();
        println!("{:?} ",i);
        res[i.len() - 1 ]+=1
    }
    
    return res;
}

fn main(){
    let logs:Vec<Vec<i32>> = vec![vec![0,5],vec![1,2],vec![0,2],vec![0,5],vec![1,3]];
    let k = 5;
    println!("{:?} ",finding_users_active_minutes(logs,k))
}