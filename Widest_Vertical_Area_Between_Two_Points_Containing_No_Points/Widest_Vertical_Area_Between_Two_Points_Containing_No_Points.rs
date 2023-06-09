fn main(){
    let points:Vec<Vec<i32>> = vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]];
    println!("{:?} ",max_width_of_vertical_area(points));
}

pub fn max_width_of_vertical_area(points:Vec<Vec<i32>>) ->i32{
    let mut v1:Vec<i32> = vec![0;points.len()];
    for i in 0..points.len(){
        v1[i]=points[i][0];
    }
    v1.sort();
    let mut max = 0;
    for i in 1..v1.len(){
        if v1[i] - v1[i-1] > max{
            max = v1[i] - v1[i-1];
        }
    }
    return max
}