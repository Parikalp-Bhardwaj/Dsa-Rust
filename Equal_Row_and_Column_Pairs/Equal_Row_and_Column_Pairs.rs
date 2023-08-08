fn equal_pairs(grid:Vec<Vec<i32>>) -> i32{
    let mut ans = 0;
    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            let mut count = 0;
            for k in 0..grid.len(){
                if grid[i][k] == grid[k][j]{
                    count+=1;
                }else{
                    break
                }
            }
            if count == grid.len(){
                ans+=1;
            }
        }
    }
    return ans
}

fn main(){
    let grid:Vec<Vec<i32>> = vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]];
    println!("{:?} ",equal_pairs(grid));
}