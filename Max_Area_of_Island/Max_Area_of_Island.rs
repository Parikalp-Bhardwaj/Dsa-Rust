// use std::cmp;

struct Solution{

}
impl Solution{
    pub fn max_area_of_island(grid:Vec<Vec<i32>>) -> i32{
        let n = grid.len();
        let m = grid[0].len();
        let mut grid = grid.clone();
        let mut area = 0;
        for i in 0..n{
            for j in 0..m{
                if grid[i][j] == 1{
                    let mut count = 0;
                    Self::dfs(i,j,&mut grid,&mut count);
                    area = area.max(count);
                }
            }
        }
        return area;
        
    }

    fn dfs(i:usize,j:usize,grid:&mut Vec<Vec<i32>>,count:&mut i32) -> (){
        if i < 0 || i >= grid.len() || j < 0 || j >= grid[0].len() || grid[i][j] == 0{
            return 
        }
        if grid[i][j] != 1{
            return;
        }
        grid[i][j] = 0;
        *count+=1;
        // return 1+ dfs(i+1,j,grid)+ dfs(i-1,j,grid)+ dfs(i,j+1,grid)+dfs(i,j-1,grid);
        Self::dfs(i+1,j,grid,count);
        Self::dfs(i-1,j,grid,count); 
        Self::dfs(i,j+1,grid,count);
        Self::dfs(i,j-1,grid,count);
    
    }
}



fn main(){
    let grid:Vec<Vec<i32>> = vec![vec![0,0,1,0,0,0,0,1,0,0,0,0,0],vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
    vec![0,1,1,0,1,0,0,0,0,0,0,0,0],vec![0,1,0,0,1,1,0,0,1,0,1,0,0],vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
    vec![0,0,0,0,0,0,0,0,0,0,1,0,0],vec![0,0,0,0,0,0,0,1,1,1,0,0,0],vec![0,0,0,0,0,0,0,1,1,0,0,0,0]];
    let mut g = Solution::max_area_of_island(grid);
    println!("{:?} ",g);
}

