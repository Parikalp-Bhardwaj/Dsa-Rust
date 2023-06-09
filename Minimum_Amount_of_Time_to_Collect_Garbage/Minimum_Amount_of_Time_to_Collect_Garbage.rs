fn  garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32{
    let mut t = vec![0;travel.len()+1];
    t.splice(1.., travel.iter().cloned());
    let trucks = vec!["G","P","M"];
    let mut count = 0;
    for i in trucks.iter(){
        let mut res = 0;
        let mut idx = 0;
        for j in 0..garbage.len(){
            let c = garbage[j].matches(i).count();
            if c > 0 {
                res+=c as i32;
                idx = j;
            }
        }
        let mut sum = 0;
        for k in 0..idx{
            sum += travel[k];
        }
        count += res + sum
    }

    return count;
}
fn main(){
    let garbage:Vec<String> = vec!["G".to_string(),"P".to_string(),"GP".to_string(),"GG".to_string()];
    let travel:Vec<i32> = vec![2,4,3];
    println!("{:?} ",garbage_collection(garbage,travel));
}