// std::fmt::Display;
// #[derive(Debug)]
fn main(){
    let names:Vec<String> = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
    let heights:Vec<i32> = vec![180, 165, 170];
    println!("{:?}, {:?}",names,heights);
    println!("{:?} ",sortPeople(names,heights));
}

fn sortPeople(names:Vec<String>,heights:Vec<i32>) ->Vec<String>{
    #[derive(Debug)]
    struct People{
        name:String,
        height:i32,
    }
    let mut res:Vec<People> = Vec::new();
    for i in 0..names.len(){
        let mut ppl = People{name:names[i].to_string(),height:heights[i]};
        res.push(ppl)
    }

    res.sort_by(|a, b| b.height.cmp(&a.height));
    let mut ans:Vec<String> = Vec::new();
    for x in 0..res.len(){
        ans.push(res[x].name.to_string());
    }
    

    return ans;
}