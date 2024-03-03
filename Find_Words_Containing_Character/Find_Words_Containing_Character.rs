struct Solution{}

impl Solution{
    fn find_words_containing(words:Vec<String>,x:char) -> Vec<i32>{
        let idx:Vec<i32> = words.clone().into_iter().enumerate().filter_map(|(index,w)|
        if w.contains(x) {
                Some(index as i32)
        }else {None}).collect();
        return idx
    }
}


fn main(){
    let words = vec!["leet","code"];
    let w = words.iter().map(|x|x.to_string()).collect::<Vec<String>>();
    let x:char = 'e';
    let sol = Solution::find_words_containing(w,x);
    println!("{:?} ",sol)
}