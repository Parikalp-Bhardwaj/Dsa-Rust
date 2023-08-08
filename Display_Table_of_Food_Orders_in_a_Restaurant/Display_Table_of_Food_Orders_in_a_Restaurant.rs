use std::collections::{BTreeSet,HashMap,BTreeMap};
use std::iter::FromIterator;
struct Solution{}


impl Solution{
    fn display_table(orders:Vec<Vec<String>>) -> Vec<Vec<String>>{
        let mut hashset_food:BTreeSet<String> =  BTreeSet::new();
        let mut hashmap = BTreeMap::<i32,HashMap<String,i32>>::new();


        for i in orders.iter(){
            let table:i32 = i[1].parse().unwrap();
            hashset_food.insert(i[2].to_string());
            *hashmap.entry(table).or_default().entry(i[2].to_string()).or_default()+=1
        }
        let mut fist_arr:Vec<String> = Vec::from_iter(hashset_food.clone().into_iter());
        let mut res = Vec::<Vec<String>>::new(); 
        fist_arr.insert(0,"Table".to_string());
        res.push(fist_arr);
        for (key,val) in hashmap.iter(){

            let mut out:Vec<String> = vec!["".to_string();1];
            out[0] = key.to_string();
  
            for j in hashset_food.iter(){
                let ans = val.get(j).unwrap_or(&0);
                out.push(ans.to_string());
                
            }
            res.push(out);
        }

        return res
    }
}

fn main(){
    let orders:Vec<Vec<String>> = vec![vec!["David".to_string(),"3".to_string(),"Ceviche".to_string()],
                                    vec!["Corina".to_string(),"10".to_string(),"Beef Burrito".to_string()],
                                    vec!["David".to_string(),"3".to_string(),"Fried Chicken".to_string()],
                                    vec!["Carla".to_string(),"5".to_string(),"Water".to_string()],
                                    vec!["Carla".to_string(),"5".to_string(),"Ceviche".to_string()],
                                    vec!["Rous".to_string(),"3".to_string(),"Ceviche".to_string()]];

    let sol = Solution::display_table(orders);
    println!("{:?}",sol);
}