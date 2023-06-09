fn main(){
    let grid:Vec<Vec<i32>> = vec![vec![0,0,1,1],vec![1,0,1,0],vec![1,1,0,0]];
    println!("{:?} ",matrix_score(grid));
}

fn matrix_score(grid:Vec<Vec<i32>>) -> i32{
    let m = grid.len();
    let n = grid[0].len();
    let mut g = grid.clone();
    for i in 0..m{
        if g[i][0] == 0{
            for j in 0..n{
                g[i][j] = 1- g[i][j];
            }
        }
    }

    for i in 0..n{
        let mut count = 0;
        for j in 0..m{
            if g[j][i] == 1{
                count+=1;
            }
        }
        if count < m-count{
            for j in 0..m{
                g[j][i] = 1 - g[j][i];
            }
        }
    }

    let mut res = 0;
    for i in 0..m{
        for j in 0..n{
            res += g[i][j] << (n-j-1);
        }
    }
    return res;
}

