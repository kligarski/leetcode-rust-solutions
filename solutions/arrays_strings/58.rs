pub fn length_of_last_word(s: String) -> i32 {
    let mut length = 0;
    for c in s.chars().rev() {
        if c == ' ' {
            if length == 0 {
                continue;
            } else {
                break;
            }
        } else {
            length += 1;
        }
    }

    length
}

fn main() {
    let string = String::from("Hello world     ");
    let expected_result = 5;

    let result = length_of_last_word(string);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
