pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut triplets: Vec<Vec<i32>> = vec![];
    for (x_index, &x_value) in nums.iter().enumerate() {
        if x_index > 0 && x_value == nums[x_index - 1] {
            continue;
        }

        let needed_sum = -x_value;

        let mut y = x_index + 1;
        let mut z = nums.len() - 1;
        while y < z {
            let sum = nums[y] + nums[z];
            if sum > needed_sum {
                z -= 1;
            } else if sum < needed_sum {
                y += 1;
            } else {
                let triple = vec![x_value, nums[y], nums[z]];

                while y < z && nums[y] == triple[1] {
                    y += 1;
                }

                while y < z && nums[z] == triple[2] {
                    z -= 1;
                }

                triplets.push(triple);
            }
        }
    }

    triplets
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let mut expected_result = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

    let mut result = three_sum(nums);

    expected_result.sort();
    result.sort();

    dbg!(&result);
    assert_eq!(expected_result, result);
}
