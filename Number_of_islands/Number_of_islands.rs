fn main(){
    let grid:Vec<Vec<char>> = vec![vec!['1','1','1','1','0'],
                                vec!['1','1','0','1','0'],
                                vec!['1','1','0','0','0'],
                                vec!['0','0','0','0','0']];

    println!("{:?} ",numIslands(grid));

}

fn numIslands(grid:Vec<Vec<char>>) ->i32 {
    let mut g = grid.clone();
    let n = g.len();
    let m = g[0].len();
    let mut count = 0;
    for i in 0..n{
        for j in 0..m{
            if g[i][j] == '1'{
                count+=1;
                numIslandsUtils(i,j,&mut g)
            }
        }
    }
    return count
}

fn numIslandsUtils(i:usize,j:usize,grid:&mut Vec<Vec<char>>) {
    grid[i][j] = '0';
    if isValid(i-1,j,grid){
        numIslandsUtils(i-1,j,grid);
    }
    if isValid(i,j-1,grid){
        numIslandsUtils(i,j-1,grid);
    }
    if isValid(i+1,j,grid){
        numIslandsUtils(i+1,j,grid);
    }
    if isValid(i,j+1,grid){
        numIslandsUtils(i,j+1,grid);
    }
    

}

fn isValid(i:usize,j:usize,grid:&mut Vec<Vec<char>>) -> bool{
    if i < grid.len() && i >= 0 && j < grid[0].len() && j >= 0 && grid[i][j] == '1'{
        return true
    }
    return false
}