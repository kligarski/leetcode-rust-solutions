fn get(vec: &Vec<i32>, index: i32) -> Option<&i32> {
    if index < 0 {
        return None;
    } else {
        return vec.get(index as usize);
    }
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut products_left: Vec<i32> = vec![1; nums.len()];
    let mut products_right: Vec<i32> = vec![1; nums.len()];
    
    for (index, &val) in nums.iter().enumerate() {
        products_left[index] = get(&products_left, index as i32 - 1).copied().unwrap_or(1) * val;
    }

    for (index_iter, &val) in nums.iter().rev().enumerate() {
        let index = nums.len() - index_iter - 1;
        products_right[index] = get(&products_right, index as i32 + 1).copied().unwrap_or(1) * val;
    }

    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        let product_left = get(&products_left, i as i32 - 1).copied().unwrap_or(1);
        let product_right = get(&products_right, i as i32 + 1).copied().unwrap_or(1);
        result.push(product_left * product_right);
    }

    result
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let expected_result = vec![24, 12, 8, 6];

    let result = product_except_self(nums);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

//  1   2   3   4
//  1   2   6  24
// 24  24  12   4
