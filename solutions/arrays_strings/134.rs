pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();

    let mut current_start = 0;
    while current_start < n && gas[current_start] - cost[current_start] < 0 {
        current_start += 1;
    }

    if current_start == n {
        return -1;
    }

    let mut current_pos = (current_start + 1) % n;
    let mut current_gas = gas[current_start] - cost[current_start];

    while current_pos < 2 * n {
        current_gas += gas[current_pos % n] - cost[current_pos % n];

        if current_gas < 0 {
            current_start = (current_pos + 1) % n;
            current_gas = 0;
        } else if (current_pos + 1) % n == current_start {
            return current_start as i32
        }

        current_pos += 1;
    }

    -1
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    let expected_result = 3;

    let result = can_complete_circuit(gas, cost);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];
    let expected_result = -1;

    let result = can_complete_circuit(gas, cost);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let gas = vec![2];
    let cost = vec![2];
    let expected_result = 0;

    let result = can_complete_circuit(gas, cost);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

//  1  2  3  4  5    gas
//  3  4  5  1  2    cost

// -2 -2 -2  3  3 