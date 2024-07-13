// Hard-coding values would be way easier and faster
pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut result = String::new();
    let orders = [
        (1000, 'M', 'x'),
        (100, 'C', 'D'),
        (10, 'X', 'L'),
        (1, 'I', 'V'),
    ];

    for (index, (order, one, five)) in orders.iter().copied().enumerate() {
        if order > num {
            continue;
        }

        let mut digit = num / order;
        if digit == 4 {
            result.push(one);
            result.push(five);
        } else if digit == 9 {
            let (_, higher_one, _) = orders[index - 1];
            result.push(one);
            result.push(higher_one);
        } else {
            while digit > 0 {
                if digit >= 5 {
                    result.push(five);
                    digit -= 5;
                } else {
                    result.push(one);
                    digit -= 1;
                }
            }
        }
        num = num % order;
    }

    result
}

fn main() {
    let number = 1994;
    let expected_result = String::from("MCMXCIV");

    let result = int_to_roman(number);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
