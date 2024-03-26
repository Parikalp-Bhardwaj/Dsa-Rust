struct Solution{}


impl Solution{
    fn replaceWords(dictionary:Vec<String>, sentence:String) -> String{
        let words:Vec<String> = sentence.split_whitespace().map(|i|i.to_string()).collect();
        let ans:Vec<String> = words.into_iter().map(|word| {
            let mut res = word.to_string();
            for dic in dictionary.iter(){
                if word.starts_with(dic) && res.len() > dic.len(){
                    res = dic.to_string();
                }
            }
            res
        }).collect();
        ans.join(" ")
    }
}


fn main(){
    let mut dictionary = vec!["cat","bat","rat"];
    let dic = dictionary.iter_mut().map(|ch| ch.to_string()).collect::<Vec<String>>();
    let sentence = "the cattle was rattled by the battery".to_string();
    let sol = Solution::replaceWords(dic,sentence); 
    println!("{:?} ",sol);

}
