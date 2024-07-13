// fn convert(s: String) -> Vec<char> {
//     s.to_ascii_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
// }

// pub fn is_palindrome(s: String) -> bool {
//     let s: Vec<char> = convert(s);
//     let n = s.len();

//     for i in 0..n/2 {
//         if s[i] != s[n - i - 1] {
//             return false;
//         }
//     }

//     true
// }

pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        while i < j && !chars[i].is_alphanumeric() {
            i += 1;
        }
        while i < j && !chars[j].is_alphanumeric() {
            j -= 1;
        }

        if i >= j {
            break;
        }

        if chars[i].to_ascii_lowercase() != chars[j].to_ascii_lowercase() {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn main() {
    let s = String::from("A man, a plan, a canal: Panama");
    let expected_result = true;

    let result = is_palindrome(s);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
