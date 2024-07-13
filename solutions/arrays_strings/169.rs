use std::mem::swap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut maj1: i32 = nums[0];
    let mut occ1: i32 = 0;

    let mut maj2: i32 = i32::MIN;
    let mut occ2: i32 = 0;

    let mut current_num = nums[0];
    let mut current_occ = 1;

    for &val in &nums[1..] {
        if val == current_num {
            current_occ += 1;
        } else {
            update_majs(
                &mut current_num,
                &mut maj1,
                &mut occ1,
                &mut current_occ,
                &mut maj2,
                &mut occ2,
                val,
            );
        }
    }
    update_majs(
        &mut current_num,
        &mut maj1,
        &mut occ1,
        &mut current_occ,
        &mut maj2,
        &mut occ2,
        i32::MIN,
    );

    maj1
}

fn update_majs(
    current_num: &mut i32,
    maj1: &mut i32,
    occ1: &mut i32,
    current_occ: &mut i32,
    maj2: &mut i32,
    occ2: &mut i32,
    val: i32,
) {
    if *current_num == *maj1 {
        *occ1 += *current_occ;
    } else if *current_num == *maj2 {
        *occ2 += *current_occ;

        if *occ2 > *occ1 {
            swap(occ1, occ2);
            swap(maj1, maj2);
        }
    } else {
        if *current_occ > *occ1 {
            *maj2 = *maj1;
            *occ2 = *occ1;
            *maj1 = *current_num;
            *occ1 = *current_occ;
        } else if *current_occ > *occ2 {
            *maj2 = *current_num;
            *occ2 = *current_occ;
        }
    }
    *current_occ = 1;
    *current_num = val;
}

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let expected_result = 2;

    let result = majority_element(nums);
    assert_eq!(result, expected_result);
    dbg!(result);

    let nums = vec![6, 5, 5];
    let expected_result = 5;

    let result = majority_element(nums);
    assert_eq!(result, expected_result);
    dbg!(result);
}
