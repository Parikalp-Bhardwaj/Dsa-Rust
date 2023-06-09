
fn main(){
    let words:Vec<String> = vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()];
    println!("{:?} ",findWords(words));
}


fn findWords(words:Vec<String>) -> Vec<String>{
    let keys = ["qwertyuiop","asdfghjkl","zxcvbnm"];
    let mut res = Vec::new();
    for word in words{
        let l = word.len();
        let mut str1 = "".to_string();
        for k in keys{
            for chars in word.chars(){
                let t = k.contains(&*&chars.to_string().to_lowercase());
                if t{
                    str1+=&chars.to_string().to_lowercase();
                }else{
                    break
                }

            }
        }
        if l == str1.len(){
            res.push(word);
        }
    }
    return res
}