pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();

    let mut i = 0;
    let mut char_to_cmp = None;
    'outer:  loop {
        if let Some(c) = strs[0].chars().nth(i) {
            char_to_cmp = Some(c);
        } else {
            break;
        }

        for str in &strs {
            if let Some(c) = str.chars().nth(i) {
                if char_to_cmp.unwrap() != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        
        prefix.push(char_to_cmp.unwrap());
        i += 1;
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let expected_result = "fl";

    let result = longest_common_prefix(strings);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
