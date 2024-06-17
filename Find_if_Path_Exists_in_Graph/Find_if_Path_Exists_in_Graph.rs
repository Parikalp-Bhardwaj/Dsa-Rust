use std::collections::{HashMap, HashSet};
struct Solution{}

impl Solution{
    fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool{
        let mut map:HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges.iter(){
            let u = edge[0];
            let v = edge[1];
            map.entry(u).or_insert(Vec::new()).push(v);
            map.entry(v).or_insert(Vec::new()).push(u);
        }
        let mut visited = HashSet::new();
        visited.insert(source);
        let mut v1 = Vec::new();
        v1.push(source);
        while !v1.is_empty(){
            let q = v1.remove(0);
            if q == destination{
                return true
            }

            if let Some(j)  = map.get(&q){
                for i in j{
                    if !visited.contains(&i){
                        visited.insert(*i);
                        v1.push(*i);
                    }
                }
            }
        }

        false
    }
}

fn main(){
    let n:i32 = 3;
    let edges:Vec<Vec<i32>> = vec![vec![0,1],vec![1,2],vec![2,0]];
    let destination:i32 = 2;
    let source = 0;
    let sol = Solution::valid_path(n,edges,source,destination);
    println!("{:?} ",sol); 
}