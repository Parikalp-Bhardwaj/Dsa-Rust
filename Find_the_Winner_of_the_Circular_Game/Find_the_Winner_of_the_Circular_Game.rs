fn findTheWinner(n:i32,k:i32) ->i32{
    let mut v1 = Vec::new();
    for i in 1..n+1{
        v1.push(i);
    }
    let mut rem = k as usize;
    while v1.len() > 1{
        rem = (rem-1) % v1.len();
        v1 = [&v1[..rem], &v1[rem+1..]].concat();
        rem=rem + (k as usize);
    }
 
    return v1[0];
}

fn main(){
    let n = 5;
    let k = 2;
    println!("{} ",findTheWinner(n,k))
}