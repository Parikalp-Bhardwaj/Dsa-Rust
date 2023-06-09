use std::collections::HashMap;
fn check_if_pangram(sentence:&str) -> bool{
    let mut hashmp:HashMap<String,bool> = HashMap::new();
    for s in sentence.chars(){
        hashmp.insert(s.to_string(),true);
    }
    if hashmp.len() == 26{
        return true;
    }
    return false;

}

fn main(){
    let sentence = "thequickbrownfoxjumpsoverthelazydog";
    println!("{:?} ",check_if_pangram(&sentence));

}

