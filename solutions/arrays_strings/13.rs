use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_chars = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    if n == 1 {
        return roman_chars[&chars[0]];
    }

    let mut number = 0;

    let mut first = roman_chars[&chars[0]];
    let mut second;

    let mut i = 0;
    while i < n - 1 {
        second = roman_chars[&chars[i + 1]];
        if first < second {
            number += second - first;
            i += 2;
            if i < n {
                first = roman_chars[&chars[i]];
            }
        } else {
            number += first;
            first = second;
            i += 1;
        }
    }

    if i == n - 1 {
        number += first;
    }

    number
}

fn main() {
    let string = String::from("MCMXCIV");
    let expected_result = 1994;

    let result = roman_to_int(string);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

// 50 5 1 1 1
// 1000 100 1000 10 100 1 5
