pub fn reverse_words(s: String) -> String {
    s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
}   

fn main() {
    let string = String::from("    hello      world  ");
    let expected_result = "world hello";

    let result = reverse_words(string);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
