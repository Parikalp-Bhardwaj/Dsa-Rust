fn main(){
    let plants:Vec<i32> = vec![2,2,3,3];
    let capacity = 5;
    println!("{} ",watering_plants(plants,capacity));
}

fn watering_plants(plants:Vec<i32>,capacity:i32) -> i32{
    let mut fill = capacity;
    let mut count:i32 = 0;
    for (i,&p) in plants.iter().enumerate(){
        if fill < p{
            fill = capacity;
            count+=2*i as i32;
        }
        fill -= p;
        count+=1;
    }
    return count;
}