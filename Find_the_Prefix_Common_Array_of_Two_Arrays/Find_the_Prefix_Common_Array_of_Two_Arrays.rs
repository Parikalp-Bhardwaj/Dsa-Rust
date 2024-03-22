struct Solution{}


impl Solution{
    fn find_the_prefix_common_array(a:Vec<i32>, b:Vec<i32>) -> Vec<i32>{
        let mut ans = vec![0;a.len()];
        let mut set = std::collections::HashSet::new();
        let mut count = 0;
        for i in 0..a.len(){
            if a[i] == b[i]{
                count+=1;
                ans[i] = count;
            }else{
                if !set.insert(a[i]){
                    count += 1;
                }
                if !set.insert(b[i]){
                    count += 1;
                } 
                ans[i] = count
            }
        }
        
        ans
    }

}


fn main(){
    let a = vec![1,3,2,4];
    let b = vec![3,1,2,4];
    let sol = Solution::find_the_prefix_common_array(a,b);
    println!("{:?} ",sol);
}