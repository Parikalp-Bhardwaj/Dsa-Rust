use std::collections::{HashMap, BTreeMap};
struct Solution{}


impl Solution{
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq:BTreeMap<char,i32> =  BTreeMap::new();

        for w in word.chars(){
            *freq.entry(w).or_insert(0)+=1;
        }         

        let mut freq_vec:Vec<(char, i32)> = freq.clone().into_iter().collect();
        freq_vec.sort_by(|a,b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let nums:Vec<i32> = vec![2,3,4,5,6,7,8,9];

        let mut res: HashMap<i32, Vec<i32>> = HashMap::new();
        for num in &nums {
            res.insert(*num, Vec::new());
        }

        for (index, &(_, val)) in freq_vec.iter().enumerate() {
            let key = 2 + (index % 8) as i32; 
            res.get_mut(&key).unwrap().push(val as i32);
        }
       
        let output:Vec<Vec<i32>> = res.values().into_iter()
                        .map(|v| {
                            v.into_iter()
                            .enumerate()
                            .map(|(index,values)| values * (index as i32 + 1))
                            .collect()
                        })
                        .collect();

        let sum = output.iter().flat_map(|v| v.iter()).sum();

        sum
    }
}

fn main(){
    // let word = "aabbccddeeffgghhiiiiii".to_string();
    // let word = "aabcdff".to_string();
    let word = "abcde".to_string();
    let sol = Solution::minimum_pushes(word);
    println!("{:?} ",sol);
}