use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let no_words = words.len();
    let word_len = words[0].len();

    let mut words_map: HashMap<String, i32> = HashMap::new();
    for word in words {
        if let Some(count) = words_map.get_mut(&word) {
            *count += 1;
        } else {
            words_map.insert(word, 1);
        }
    }

    let mut answer_indexes: Vec<i32> = vec![];

    for starting_pos in 0..word_len {
        let mut left = starting_pos;
        let mut right = left;

        let mut window_len = 0;
        let mut window_map: HashMap<String, i32> = HashMap::new();

        while right + word_len <= s.len() {
            right += word_len;
            let next_word = String::from(&s[right - word_len..right]);

            let mut move_left = false;
            if let Some(expected_count) = words_map.get(&next_word) {
                if let Some(window_count) = window_map.get_mut(&next_word) {
                    if *window_count < *expected_count {
                        *window_count += 1;
                        window_len += 1;
                    } else {
                        right -= word_len;
                        move_left = true;
                    }
                } else {
                    window_map.insert(next_word.clone(), 1);
                    window_len += 1;
                }
            } else {
                left = right;
                window_len = 0;
                window_map = HashMap::new();
                continue;
            }

            if window_len == no_words {
                answer_indexes.push(left as i32);
                move_left = true
            }

            if move_left {
                let prev_word = String::from(&s[left..left + word_len]);
                *window_map.get_mut(&prev_word).unwrap() -= 1;
                left += word_len;
                window_len -= 1;
            }
        }
    }
    answer_indexes
}

fn main() {
    let s = String::from("barfoofoobarthefoobarman");
    let words = vec![
        String::from("bar"),
        String::from("foo"),
        String::from("the"),
    ];
    let expected_result = vec![6, 9, 12];

    let result = find_substring(s, words);

    dbg!(&result);
    assert_eq!(expected_result, result);
}

// [aba, baa]
// baababaa

// [abab, baab, abba]
// ababbaababababbaababba
// xxxxyyyy
//   zzzzxxxx

// [aba, bab]
// abababababa
// xxx
// xxxyyy
//    yyyxxx
//       xxx
//  yyyxxx       but this kind of sliding window will skip these:
//     xxxyyy

// if l = length of a word
// we need to start from every position 0, 1, ..., l - 1
// and do regular sliding window

// O(n*l)
