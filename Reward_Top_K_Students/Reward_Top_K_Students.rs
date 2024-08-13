use std::collections::{BTreeMap, HashSet};
use std::convert::TryInto;
struct Solution{}


impl Solution{

    pub fn top_students(positive_feedback: Vec<String>, negative_feedback: Vec<String>, report: Vec<String>, student_id: Vec<i32>, k: i32) -> Vec<i32> {
        let positive_feedback: HashSet<_> =  positive_feedback.into_iter().collect();
        let negative_feedback: HashSet<_> = negative_feedback.into_iter().collect();
        let mut score:BTreeMap<i32,i32> = BTreeMap::new(); 
        for &id in &student_id {
            score.entry(id).or_insert(0);
        }

        for (report, &student_id) in report.iter().zip(student_id.iter()) {
            for word in report.split_whitespace() {
                if positive_feedback.contains(&word.to_string()) {
                    *score.entry(student_id).or_insert(0) += 3;
                } else if negative_feedback.contains(&word.to_string()) {
                    *score.entry(student_id).or_insert(0) -= 1;
                }
            }
        }

        let mut score_vec: Vec<(i32, i32)> = score.into_iter().collect();  
        score_vec.sort_by(|a, b| {
            b.1.cmp(&a.1) 
            .then_with(|| a.0.cmp(&b.0)) 
        });
        score_vec.into_iter().map(|(id, _)| id).take(k.try_into().unwrap()).collect()
      
    }
}


fn main(){
    let mut positive_feedback:Vec<String> = vec!["smart".to_string(),"brilliant".to_string(),"studious".to_string()];
    // positive_feedback.i().map(|s|s.to_string()).collect::<Vec<String>>().join(" ");
    let mut negative_feedback: Vec<String> = vec!["not".to_string()];
    // negative_feedback.iter_mut().map(|s|s.to_string()).collect::<Vec<String>>().join(" ");
    let mut report: Vec<String> = vec!["this student is not studious".to_string(),"the student is smart".to_string()];
    // report.iter_mut().map(|s|s.to_string()).collect::<Vec<String>>().join(" ");
    let student_id: Vec<i32> = vec![1,2];
    let k:i32 = 2;
    let sol = Solution::top_students(positive_feedback,negative_feedback,report,student_id, k);
    println!("{:?} ",sol);

    

}