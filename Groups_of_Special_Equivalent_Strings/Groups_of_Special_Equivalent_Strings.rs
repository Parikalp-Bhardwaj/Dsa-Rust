use std::collections::HashSet;
fn main(){
    let words = vec!["abcd","cdab","cbad","xyzz","zzxy","zzyx"]
    .iter()
    .map(|s|s.to_string()).collect::<Vec<String>>();
    println!("{:?} ",num_special_equiv_groups(words));
}

fn num_special_equiv_groups(words:Vec<String>) -> i32{
    let mut hashset:HashSet::<String> = HashSet::new(); 
    for word in words{
        let mut even:Vec<char> = Vec::new();
        let mut odd:Vec<char> = Vec::new();
        for (idx,w) in word.chars().enumerate(){
            if idx % 2 == 0{
                even.push(w);
            }else{
                odd.push(w);
            }
        }
        even.sort();
        odd.sort();
        let my_str  = format!("{}{}",even.iter().collect::<String>(),
        odd.iter().collect::<String>());
        hashset.insert(my_str);
    }
    let out = hashset.len() as i32;
    out
}