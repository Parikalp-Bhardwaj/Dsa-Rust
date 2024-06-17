use std::collections::HashSet;

struct Solution{}


impl Solution{
    fn find_center(edges: Vec<Vec<i32>>) -> i32{
        let mut hashset = HashSet::new();
        // hashset.insert(edges[0][0]);
        // hashset.insert(edges[0][1]);
        for edge in edges.iter().skip(1){
            let i = edge[0];
            let j = edge[1];
            
            if  hashset.contains(&i){
                return i;
            }else if hashset.contains(&j){
                return j;
            }else{
                hashset.insert(i);
                hashset.insert(j);
                
            }
        }
        0
    }
}

fn main(){
    let edges:Vec<Vec<i32>> = vec![vec![1,2],vec![2,3],vec![4,2]];
    let sol = Solution::find_center(edges);
    println!("{:?} ",sol);
}