pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let num_rows = num_rows as usize;
    let mut result = String::new();
    let delta = num_rows - 2;
    let chars: Vec<char> = s.chars().collect();

    for row in 0..num_rows {
        let mut i = row;
        while i < s.len() {
            result.push(chars[i]);

            if row != 0 && row != num_rows - 1 {
                i += delta + num_rows - row - row;
                if i >= s.len() {
                    break;
                }

                result.push(chars[i]);

                i += row + row;
            } else {
                i += delta + num_rows;
            }
        }
    }

    result
}

// enum Direction {
//     UP,
//     DOWN
// }

// pub fn convert(s: String, num_rows: i32) -> String {
//     if num_rows == 1 {
//         return s;
//     }

//     let num_rows = num_rows as usize;
//     let mut strings = vec![String::new(); num_rows];

//     let mut pos = 0;
//     let mut direction = Direction::DOWN;
//     for i in 0..s.len() {
//         strings[pos].push(s.chars().nth(i).unwrap());

//         match direction {
//             Direction::DOWN => {
//                 if pos == num_rows - 1 {
//                     pos -= 1;
//                     direction = Direction::UP;
//                 } else {
//                     pos += 1;
//                 }
//             },
//             Direction::UP => {
//                 if pos == 0 {
//                     pos += 1;
//                     direction = Direction::DOWN;
//                 } else {
//                     pos -= 1;
//                 }
//             }
//         }
//     }

//     strings.join("")
// }

fn main() {
    let string = String::from("PAYPALISHIRING");
    let expected_result = "PINALSIGYAHRPI";

    let result = convert(string, 4);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

// P       H
// A     S I
// Y   I   R
// P L     I G
// A       N
// num_rows = 5, delta = 3
