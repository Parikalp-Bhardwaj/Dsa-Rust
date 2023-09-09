use std::cmp;
struct Solution {}

impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut min = prices[0];
        for i in prices.into_iter().skip(1) {
            res = cmp::max(res, i - min);
            min = cmp::min(min, i);
        }
        return res;
    }
}

fn main() {
    let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    let sol = Solution::max_profit(prices);
    println!("{:?} ", sol);
}
