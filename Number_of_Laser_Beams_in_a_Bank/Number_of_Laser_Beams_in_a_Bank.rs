fn main(){
    let bank:Vec<String> = vec!["011001".to_string(),"000000".to_string(),"010100".to_string(),"001000".to_string()];
    println!("{:?} ",numberOfBeams(bank));
}

fn numberOfBeams(bank:Vec<String>) -> i32{
    let mut res = 0;
    let mut prev = 0;
    for b1 in bank{
        let count = b1.matches("1").count() as i32;
        match count{
            count if count > 0 => {res += count *prev;
            prev = count
        },
        _ => (),
        }
    }
    return res
}