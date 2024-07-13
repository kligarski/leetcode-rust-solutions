fn get_or(v: &Vec<i32>, index: i32) -> i32 {
    if index < 0 {
        return i32::MAX;
    } else {
        v.get(index as usize).unwrap_or(&i32::MIN).to_owned()
    }
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut pos = 0;
    while pos < prices.len() {
        // a >= b < c
        while pos < prices.len()
            && (get_or(&prices, pos as i32 - 1) < prices[pos]
                || prices[pos] >= get_or(&prices, pos as i32 + 1))
        {
            pos += 1;
        }

        if pos >= prices.len() {
            break;
        }

        let buy_price = prices[pos];
        pos += 1;

        // a <= b > c
        while pos < prices.len()
            && (get_or(&prices, pos as i32 - 1) > prices[pos]
                || prices[pos] <= get_or(&prices, pos as i32 + 1))
        {
            pos += 1;
        }

        if pos >= prices.len() {
            break;
        }

        profit += prices[pos] - buy_price;
        pos += 1;
    }

    profit
}

fn main() {
    let nums = vec![7, 1, 5, 3, 6, 4];
    let expected_result = 7;

    let result = max_profit(nums);
    assert_eq!(result, expected_result);
    dbg!(result);
}
