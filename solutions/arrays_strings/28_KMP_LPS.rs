fn generate_LPS(needle: &Vec<char>) -> Vec<usize> {
    let mut lps = vec![0; needle.len()];

    let mut prev = 0;
    let mut i = 1;
    while i < lps.len() {
        if needle[i] == needle[prev] {
            lps[i] = prev + 1;
            i += 1;
            prev += 1;
        } else if prev == 0 {
            // lps[i] = 0;
            i += 1;
        } else {
            prev = lps[prev - 1];
        }
    }
    lps
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();

    let lps = generate_LPS(&needle);

    let mut i = 0;
    let mut j = 0;
    while i < haystack.len() {
        if haystack[i] == needle[j] {
            i += 1;
            j += 1;
        } else if j == 0 {
            i += 1;
        } else {
            j = lps[j - 1];
        }

        if j == needle.len() {
            return (i - needle.len()) as i32;
        }
    }
    -1
}

fn main() {
    let string = String::from("abcdsasadsad");
    let expected_result = 6;

    let result = str_str(string, String::from("sad"));

    dbg!(&result);
    assert_eq!(expected_result, result);
}
