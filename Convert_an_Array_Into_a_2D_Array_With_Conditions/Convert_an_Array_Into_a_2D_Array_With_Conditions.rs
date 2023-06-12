use std::collections::BTreeMap;
#[derive(Debug)]
struct Solution {}

impl Solution {
    fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut btreemap: BTreeMap<i32, i32> = BTreeMap::new();
        for num in nums.iter() {
            *btreemap.entry(*num).or_default() += 1;
        }
        let max = *btreemap.values().max().unwrap();
        let mut out: Vec<Vec<i32>> = vec![vec![]; max as usize];
        btreemap.into_iter().for_each(|(k, v)| {
            (0..v).for_each(|i| {
                out[i as usize].push(k);
            });
        });
        return out;
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 3, 4, 1, 2, 3, 1];
    println!("{:?} ", Solution::find_matrix(nums));
}
