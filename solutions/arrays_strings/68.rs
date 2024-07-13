use std::collections::VecDeque;

fn add_spaces(line: &mut String, max_width: usize) {
    let spaces_to_add = max_width - line.len();
    line.push_str(&" ".repeat(spaces_to_add));
}

pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut words = VecDeque::from(words);
    let max_width = max_width as usize;

    let mut output: Vec<String> = Vec::new();

    while words.len() > 0 {
        let mut line_words: Vec<String> = Vec::new();
        let mut line_words_length = 0;
        while words.len() > 0 && line_words_length + line_words.len() + words[0].len() <= max_width {
            let word = words.pop_front().unwrap();
            line_words_length += word.len();
            line_words.push(word);
        }

        if words.len() == 0 || line_words.len() == 1 {
            let mut line = line_words.join(" ");
            add_spaces(&mut line, max_width);
            output.push(line);
            continue;
        }

        let spaces_to_justify = max_width - line_words_length;
        let gaps = line_words.len() - 1;
        let spaces_per_gap = spaces_to_justify / gaps;
        let mut additional_spaces = (spaces_to_justify % gaps) as i32;
        
        let mut line_words = VecDeque::from(line_words);
        let mut line = line_words.pop_front().unwrap();

        while line_words.len() > 0 {
            line.push_str(&" ".repeat(spaces_per_gap));
            if additional_spaces > 0 {
                line.push(' ');
                additional_spaces -= 1;
            }
            line.push_str(&line_words.pop_front().unwrap());
        }
        output.push(line);
    }

    output
}

fn main() {
    let words = vec!["What", "must", "be", "acknowledgment", "shall", "be"]
        .into_iter()
        .map(String::from)
        .collect();
    let expected_result: Vec<String> =
        vec!["What   must   be", "acknowledgment  ", "shall be        "]
            .into_iter()
            .map(String::from)
            .collect();

    let result = full_justify(words, 16);

    dbg!(&result);
    assert_eq!(expected_result, result);

    let words = vec![
        "Science",
        "is",
        "what",
        "we",
        "understand",
        "well",
        "enough",
        "to",
        "explain",
        "to",
        "a",
        "computer.",
        "Art",
        "is",
        "everything",
        "else",
        "we",
        "do",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let expected_result: Vec<String> = vec![
        "Science  is  what we",
        "understand      well",
        "enough to explain to",
        "a  computer.  Art is",
        "everything  else  we",
        "do                  ",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let result = full_justify(words, 20);

    dbg!(&result);
    assert_eq!(expected_result, result);
}
