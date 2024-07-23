pub fn min_window(s: String, t: String) -> String {
    let mut chars_to_find: [i32; 128] = [0; 128];
    let s_chars: Vec<char> = s.chars().collect();

    let mut chars_to_satisfy = 0;
    for c in t.chars() {
        if chars_to_find[c as usize] == 0 {
            chars_to_satisfy += 1;
        }
        chars_to_find[c as usize] += 1;
    }

    let mut answer: Option<&str> = None;

    let mut left = 0;
    let mut right = 0;

    while right <= s.len() {
        if chars_to_satisfy == 0 {
            let prev_char = s_chars[left];
            left += 1;

            chars_to_find[prev_char as usize] += 1;
            if chars_to_find[prev_char as usize] > 0 {
                chars_to_satisfy += 1;
            }
        } else if right + 1 <= s.len() {
            let next_char = s_chars[right];
            right += 1;

            chars_to_find[next_char as usize] -= 1;
            if chars_to_find[next_char as usize] == 0 {
                chars_to_satisfy -= 1;
            }
        } else {
            break;
        }

        if chars_to_satisfy == 0 && (answer.is_none() || answer.unwrap().len() > right - left) {
            answer = Some(&s[left..right]);
        }
    }

    if answer.is_none() {
        String::from("")
    } else {
        String::from(answer.unwrap())
    }
}

fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    let expected_result = String::from("BANC");

    let result = min_window(s, t);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
