struct Solution{}


impl Solution{
    fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
        let mut i = 0;
        let mut j = 0;
        let mut v1 = Vec::new(); 
        while i < first_list.len() && j < second_list.len(){
            let x = &first_list[i];
            let y = &second_list[j];
            let start = std::cmp::max(x[0],y[0]);
            let end = std::cmp::min(x[1],y[1]);
            if start <= end{
                v1.push(vec![start,end]);
            } 

            if x[1] < y[1]{
                i+=1;
            }else{
                j+=1
            }
        }
        v1
    }
}

fn main(){
    let first_list = vec![[0,2],[5,10],[13,23],[24,25]]
    .iter().map(|s| vec![s[0],s[1]]).collect::<Vec<Vec<i32>>>();
    let second_list = [[1,5],[8,12],[15,24],[25,26]]
    .iter().map(|s| vec![s[0],s[1]]).collect::<Vec<Vec<i32>>>();
     
    println!("{:?} ",Solution::interval_intersection(first_list,second_list));
}