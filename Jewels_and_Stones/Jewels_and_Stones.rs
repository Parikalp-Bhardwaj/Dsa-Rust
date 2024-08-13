struct Solution{}


impl Solution{
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut count = 0;
        for i in stones.chars(){
            if jewels.contains(i){
                count += 1;
            }
        }
        count
    }
}


fn main(){
    let jewels = "aA".to_string(); 
    let stones = "aAAbbbb".to_string();
    let sol = Solution::num_jewels_in_stones(jewels,stones);
    println!("{:?} ",sol);

}