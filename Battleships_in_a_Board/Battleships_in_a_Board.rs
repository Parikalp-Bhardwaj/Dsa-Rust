fn main(){
    let board:Vec<Vec<String>> = vec![vec!["X".to_string(),".".to_string(),".".to_string(),"X".to_string()],vec![".".to_string(),".".to_string(),".".to_string(),"X".to_string()],vec![".".to_string(),".".to_string(),".".to_string(),"X".to_string()]];
    println!("{:?} ",countBattleships(board));
}

fn countBattleships(board:Vec<Vec<String>>) ->i32{
    let mut res = 0;
    for x in 0..board.len(){
        for y in 0..board[x].len(){
            if board[x][y] == ".".to_string(){
                continue
            } 
            if x > 0 && board[x-1][y] == "X".to_string(){
                continue
            }
            if y > 0 && board[x][y-1] == "X".to_string(){
                continue
            }
            res +=1; 
        }
    } 
    println!("{} ",res);
    return 0;
}