pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }
    
    let sequence_to_find: Vec<char> = s.chars().collect();
    let string: Vec<char> = t.chars().collect();

    let mut s_pos = 0;
    for c in string {
        if c == sequence_to_find[s_pos] {
            s_pos += 1;
        }
        if s_pos == sequence_to_find.len() {
            return true;
        }
    }
    false
}

fn main() {
    let s = String::from("abc");
    let expected_result = true;

    let result = is_subsequence(s, String::from("ahbgdc"));

    dbg!(&result);
    assert_eq!(expected_result, result);
}
