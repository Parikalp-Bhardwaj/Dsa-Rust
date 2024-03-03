struct Solution{}


impl Solution{
    fn reverse_parentheses(s: String) -> i32{
       let mut v1 = vec![String::new()];
       let mut count = 0;
       for i in s.chars(){
        if i == '('{
            v1.push(String::new());
            count+=1;
        }else if i == ')' {
            let l = v1.pop().unwrap().chars().rev().collect::<String>();

            v1[count - 1].push_str(&l);                

            count -= 1;
        }else{
            v1[count].push(i);
        }
       }
       println!("{:?} ",v1[0].clone());
        return 0;
       
    }
}


fn main(){
    // let s = "(abcd)".to_string();
    let s ="(u(love)i)".to_string();
    let sol = Solution::reverse_parentheses(s);
    println!("{} ",sol);
}