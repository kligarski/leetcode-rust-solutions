use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }

    let mut current_chars: HashSet<char> = HashSet::new();
    let s: Vec<char> = s.chars().collect();

    let mut answer = 1;
    current_chars.insert(s[0]);

    let mut left = 0;
    let mut right = 0;

    while right + 1 < s.len() {
        let next_char = s[right + 1];
        if current_chars.contains(&next_char) {
            while s[left] != next_char {
                current_chars.remove(&s[left]);
                left += 1;
            }
            if left != right + 1 {
                left += 1;
            }
        }
        current_chars.insert(next_char);
        right += 1;
        answer = std::cmp::max(answer, right - left + 1);
    }

    answer as i32
}

fn main() {
    let s = String::from("pwwkew");
    let expected_result = 3; // wke

    let result = length_of_longest_substring(s);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
