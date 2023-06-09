fn main(){
    let queries = vec!["FooBar".to_string(), "FooBarTest".to_string(), "FootBall".to_string(), "FrameBuffer".to_string(), "ForceFeedBack".to_string()];
	let pattern = "FoBa".to_string();
    println!("{:?} ",camel_match(queries,pattern))
}

fn camel_match(queries:Vec<String>,pattern:String) ->Vec<bool>{
    let mut v1:Vec<bool> = Vec::new();
    for query in queries{
        let mut count = 0;
        let mut flag = 0;
        for i in 0..query.len(){
            if count < pattern.len(){
                if query.chars().nth(i).unwrap() == pattern.chars().nth(count).unwrap(){
                    count+=1
                }else if query.chars().nth(i).unwrap() == query.to_uppercase().chars().nth(i).unwrap(){
                    flag = 1;
                    break
                }
            }else if query[i..] != query[i..].to_lowercase(){
                flag = 1;
                break
            }else{
                break
            }

        }
        if flag == 0 && count == pattern.len(){
            v1.push(true);
        }else{
            v1.push(false);
        }
    }
    
    return v1;
}