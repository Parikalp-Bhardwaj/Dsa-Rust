use std::collections::HashMap;
struct Solution{}


impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut count = 0;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::num_islands_rec(&mut grid, i, j, n, m);
                }
            }
        }
        count
    }

    fn num_islands_rec(grid: &mut Vec<Vec<char>>, i: usize, j: usize, n: usize, m: usize) {
        if i >= n || j >= m || grid[i][j] != '1' {
            return;
        }
        grid[i][j] = '0';  

        if i > 0 {
            Self::num_islands_rec(grid, i - 1, j, n, m);
        }
        if i + 1 < n {
            Self::num_islands_rec(grid, i + 1, j, n, m);
        }
        if j > 0 {
            Self::num_islands_rec(grid, i, j - 1, n, m);
        }
        if j + 1 < m {
            Self::num_islands_rec(grid, i, j + 1, n, m);
        }
    }
}

fn main(){
    let grid:Vec<Vec<char>> = vec![
        vec!['1','1','1','1','0'],
        vec!['1','1','0','1','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','0','0','0']
      ];

    let sol = Solution::num_islands(grid);
    println!("{:?} ",sol);
}