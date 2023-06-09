use std::collections::HashMap;

fn main(){
    let list1:Vec<String> = ["Shogun".to_string(),"Tapioca Express".to_string(),"Burger King".to_string(),"KFC".to_string()].to_vec();
    let list2:Vec<String> = ["Piatti".to_string(),"The Grill at Torrey Pines".to_string(),"Hungry Hunter Steakhouse".to_string(),"Shogun".to_string()].to_vec();
    println!("{:?} ",find_restaurant(list1,list2)); 
}


fn find_restaurant(list1:Vec<String>,list2:Vec<String>) -> Vec<String>{
    let mut l1= HashMap::new();    
    for (i,s) in list1.iter().enumerate(){
        l1.insert(s,i);
    }
    
    let mut v1:Vec<String> = Vec::new();
    let mut min = list1.len() + list2.len();
    for (i,s) in list2.iter().enumerate(){
        if let Some(idx) = l1.get(s){
            if i+idx == min{
                v1.push(s.clone());
            }else if i + idx < min {
            v1.clear();
            v1.push(s.clone());
            min = i+idx;
            }
        }
    }


    return v1;
}