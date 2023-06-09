use std::cmp;
fn main(){
    let grid:Vec<Vec<i32>> = vec![vec![3,0,8,4],vec![2,4,5,7],vec![9,2,6,3],vec![0,3,1,0]];
    println!("{} ",maxIncreaseKeepingSkyline(grid));
}

fn maxIncreaseKeepingSkyline(grid:Vec<Vec<i32>>) -> i32{
    let mut col:Vec<i32> = vec![0;grid.len()];
    let mut row:Vec<i32> = vec![0;grid.len()];
    
    println!("{:?} ",col);
    for i in 0..grid.len(){
        for j in 0..grid.len(){
            row[i] = cmp::max(row[i],grid[i][j]);
            col[j] = cmp::max(col[j],grid[i][j]);
        }
    }

    let mut out = 0;
    for x in 0..grid.len(){
        for y in 0..grid.len(){
            out += cmp::min(row[x],col[y]) - grid[x][y] as i32;
        }
    }


    return out;
}