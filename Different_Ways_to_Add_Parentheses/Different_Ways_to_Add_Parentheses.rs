fn diffWaysToCompute(expression:String) -> Vec<i32>{
    let mut v1:Vec<i32> = Vec::new();
    
    for (i,s) in expression.chars().enumerate(){
        if s == '+' || s == '-' || s == '*'{
            let left = diffWaysToCompute((&expression[0..i]).to_string());
            let right = diffWaysToCompute((&expression[i+1..]).to_string());
            for x in left.iter(){
                for y in right.iter(){
                    match s{
                        '+' => v1.push(x+y),
                        '-' => v1.push(x-y),
                        '*' => v1.push(x*y),
                        _ => {}
                    }
                }
            }
        }
    }
    if v1.is_empty(){
        v1.push(expression.parse::<i32>().unwrap())
    }
    return v1
}

fn main(){
    let expression = "2-1-1".to_string();
    println!("{:?} ",diffWaysToCompute(expression));
}