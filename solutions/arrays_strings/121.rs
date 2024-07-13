pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut curr_min = prices[0];
    let mut curr_max_profit = 0;

    for &price in &prices[1..] {
        if price < curr_min {
            curr_min = price;
        }

        if price - curr_min > curr_max_profit {
            curr_max_profit = price - curr_min;
        }
    }

    curr_max_profit
}

fn main() {
    let nums = vec![7, 1, 5, 3, 6, 4];
    let expected_result = 5;

    let result = max_profit(nums);
    assert_eq!(result, expected_result);
    dbg!(result);
}
