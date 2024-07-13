fn get(v: &Vec<i32>, index: i32) -> i32 {
    if index < 0 {
        return 0;
    } else {
        return v.get(index as usize).copied().unwrap_or(0);
    }
}

fn traverse(
    height: &Vec<i32>,
    reverse: bool,
    left: fn(usize) -> i32,
    right: fn(usize) -> i32,
    cmp: fn(i32, i32) -> bool,
) -> i32 {
    let mut result = 0;

    let mut current_height = 0;
    let mut current_water = 0;

    let range: Box<dyn Iterator<Item = usize>> = if reverse {
        Box::new((0..height.len()).rev())
    } else {
        Box::new(0..height.len())
    };

    for i in range {
        if get(&height, left(i)) <= height[i] && height[i] > get(&height, right(i)) {
            if current_height == 0 {
                current_height = height[i];
                current_water = 0;
                continue;
            } else if cmp(height[i], current_height) {
                result += current_water;
                current_water = 0;
                current_height = height[i];
                continue;
            }
        }

        if current_height > height[i] {
            current_water += current_height - height[i];
        }
    }

    result
}

pub fn trap(height: Vec<i32>) -> i32 {
    let add = |x: usize| x as i32 + 1;
    let sub = |x: usize| x as i32 - 1;

    let inclusive = |x, y| x >= y;
    let exclusive = |x, y| x > y;

    let left = traverse(&height, false, sub, add, inclusive);
    let right = traverse(&height, true, add, sub, exclusive);

    left + right
}

fn main() {
    let nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let expected_result = 6;

    let result = trap(nums);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let nums = vec![4, 2, 0, 3, 2, 5];
    let expected_result = 9;

    let result = trap(nums);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

// 0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1
//    x     x           x        x
//       1     1  2  1
//                            1

// 4,2,0,3,2,5
// x     x   x
//   2 4 1 2

// 4,2,0,3,2
// x     x
//   2 4 1 2 no local maxima at the end - we ignore this
//   1 3

