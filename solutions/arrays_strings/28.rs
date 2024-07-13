// pub fn str_str(haystack: String, needle: String) -> i32 {
//     match haystack.find(&needle) {
//         Some(i) => i as i32,
//         None => -1,
//     }
// }

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();

    let mut i = 0;
    while i < haystack.len() {
        while i < haystack.len() && haystack[i] != needle[0] {
            i += 1;
        }
        
        let start = i;
        let mut j = 0;
        while i < haystack.len() && j < needle.len() && haystack[i] == needle[j] {
            i += 1;
            j += 1;
        }

        if j == needle.len() {
            return start as i32;
        } else if i < haystack.len() {
            i = start + 1;
        }
    }
    -1
}

fn main() {
    let string = String::from("sadbutsad");
    let expected_result = 0;

    let result = str_str(string, String::from("sad"));

    dbg!(&result);
    assert_eq!(expected_result, result);
}